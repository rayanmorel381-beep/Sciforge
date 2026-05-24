use std::{collections::HashMap, sync::Arc};

use sqlx::{Row, postgres::PgPoolOptions};
use tokio::sync::RwLock;
use uuid::Uuid;

use server_control::{ServerState, ServerSummary, UpdateServerRequest, Visibility};

#[derive(Clone)]
pub enum ServerStore {
    Memory(Arc<RwLock<HashMap<Uuid, ServerSummary>>>),
    Postgres(sqlx::PgPool),
}

impl ServerStore {
    pub async fn from_env() -> Self {
        match std::env::var("SERVER_MANAGER_DATABASE_URL") {
            Ok(url) if !url.trim().is_empty() => {
                let pool = PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&url)
                    .await
                    .unwrap();
                sqlx::query(
                    "create table if not exists game_servers (id uuid primary key, owner_account_id uuid not null, link_code text not null, join_code text not null unique, access_code text, realtime_endpoint text not null, name text not null, region text not null, mode text not null, visibility text not null, max_players integer not null, state text not null, created_at timestamptz not null, updated_at timestamptz not null)",
                )
                .execute(&pool)
                .await
                .unwrap();
                sqlx::query("alter table game_servers add column if not exists access_code text")
                    .execute(&pool)
                    .await
                    .unwrap();
                sqlx::query("alter table game_servers add column if not exists realtime_endpoint text not null default ''")
                    .execute(&pool)
                    .await
                    .unwrap();
                Self::Postgres(pool)
            }
            _ => Self::Memory(Arc::new(RwLock::new(HashMap::new()))),
        }
    }

    pub async fn insert(&self, server: ServerSummary) {
        match self {
            Self::Memory(store) => {
                store.write().await.insert(server.id, server);
            }
            Self::Postgres(pool) => {
                sqlx::query(
                    "insert into game_servers (id, owner_account_id, link_code, join_code, access_code, realtime_endpoint, name, region, mode, visibility, max_players, state, created_at, updated_at) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
                )
                .bind(server.id)
                .bind(server.owner_account_id)
                .bind(server.link_code)
                .bind(server.join_code)
                .bind(server.access_code)
                .bind(server.realtime_endpoint)
                .bind(server.name)
                .bind(server.region)
                .bind(server.mode)
                .bind(visibility_to_str(&server.visibility))
                .bind(server.max_players as i32)
                .bind(state_to_str(&server.state))
                .bind(server.created_at)
                .bind(server.updated_at)
                .execute(pool)
                .await
                .unwrap();
            }
        }
    }

    pub async fn list(&self, owner_account_id: Option<Uuid>, visibility: Option<Visibility>) -> Vec<ServerSummary> {
        let mut servers = match self {
            Self::Memory(store) => store.read().await.values().cloned().collect::<Vec<_>>(),
            Self::Postgres(pool) => sqlx::query(
                "select id, owner_account_id, link_code, join_code, access_code, realtime_endpoint, name, region, mode, visibility, max_players, state, created_at, updated_at from game_servers",
            )
            .fetch_all(pool)
            .await
            .unwrap()
            .into_iter()
            .map(map_server)
            .collect::<Vec<_>>(),
        };

        if let Some(owner) = owner_account_id {
            servers.retain(|server| server.owner_account_id == owner);
        }
        if let Some(value) = visibility {
            servers.retain(|server| server.visibility == value);
        }
        servers
    }

    pub async fn get(&self, server_id: Uuid) -> Option<ServerSummary> {
        match self {
            Self::Memory(store) => store.read().await.get(&server_id).cloned(),
            Self::Postgres(pool) => sqlx::query(
                "select id, owner_account_id, link_code, join_code, access_code, realtime_endpoint, name, region, mode, visibility, max_players, state, created_at, updated_at from game_servers where id = $1",
            )
            .bind(server_id)
            .fetch_optional(pool)
            .await
            .unwrap()
            .map(map_server),
        }
    }

    pub async fn update(&self, server_id: Uuid, payload: UpdateServerRequest) -> Option<ServerSummary> {
        match self {
            Self::Memory(store) => {
                let mut guard = store.write().await;
                let server = guard.get_mut(&server_id)?;
                apply_update(server, payload);
                Some(server.clone())
            }
            Self::Postgres(pool) => {
                let mut server = self.get(server_id).await?;
                apply_update(&mut server, payload);
                sqlx::query(
                    "update game_servers set visibility = $2, max_players = $3, state = $4, updated_at = $5 where id = $1",
                )
                .bind(server.id)
                .bind(visibility_to_str(&server.visibility))
                .bind(server.max_players as i32)
                .bind(state_to_str(&server.state))
                .bind(server.updated_at)
                .execute(pool)
                .await
                .unwrap();
                Some(server)
            }
        }
    }
}

fn apply_update(server: &mut ServerSummary, payload: UpdateServerRequest) {
    if let Some(visibility) = payload.visibility {
        server.visibility = visibility;
    }
    if let Some(state) = payload.state {
        server.state = state;
    }
    if let Some(max_players) = payload.max_players {
        server.max_players = max_players.clamp(2, 128);
    }
    server.updated_at = chrono::Utc::now();
}

fn map_server(row: sqlx::postgres::PgRow) -> ServerSummary {
    ServerSummary {
        id: row.get("id"),
        owner_account_id: row.get("owner_account_id"),
        link_code: row.get("link_code"),
        join_code: row.get("join_code"),
        access_code: row.get("access_code"),
        realtime_endpoint: row.get("realtime_endpoint"),
        name: row.get("name"),
        region: row.get("region"),
        mode: row.get("mode"),
        visibility: parse_visibility(row.get::<String, _>("visibility")),
        max_players: row.get::<i32, _>("max_players") as u16,
        state: parse_state(row.get::<String, _>("state")),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn visibility_to_str(value: &Visibility) -> &'static str {
    match value {
        Visibility::Public => "public",
        Visibility::Private => "private",
        Visibility::InviteOnly => "invite_only",
        Visibility::Unlisted => "unlisted",
    }
}

fn state_to_str(value: &ServerState) -> &'static str {
    match value {
        ServerState::Pending => "pending",
        ServerState::Online => "online",
        ServerState::Stopped => "stopped",
        ServerState::Failed => "failed",
    }
}

fn parse_visibility(value: String) -> Visibility {
    match value.as_str() {
        "private" => Visibility::Private,
        "invite_only" => Visibility::InviteOnly,
        "unlisted" => Visibility::Unlisted,
        _ => Visibility::Public,
    }
}

fn parse_state(value: String) -> ServerState {
    match value.as_str() {
        "pending" => ServerState::Pending,
        "stopped" => ServerState::Stopped,
        "failed" => ServerState::Failed,
        _ => ServerState::Online,
    }
}