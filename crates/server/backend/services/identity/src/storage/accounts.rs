use std::{collections::HashMap, sync::Arc};

use sqlx::{Row, postgres::PgPoolOptions};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::models::AccountRecord;

#[derive(Clone)]
pub enum AccountStore {
    Memory(Arc<RwLock<HashMap<Uuid, AccountRecord>>>),
    Postgres(sqlx::PgPool),
}

impl AccountStore {
    pub async fn from_env() -> Self {
        match std::env::var("IDENTITY_DATABASE_URL") {
            Ok(url) if !url.trim().is_empty() => {
                let pool = PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&url)
                    .await
                    .unwrap();
                sqlx::query(
                    "create table if not exists identity_accounts (id uuid primary key, username text not null unique, device_id text not null, created_at timestamptz not null default now())",
                )
                .execute(&pool)
                .await
                .unwrap();
                Self::Postgres(pool)
            }
            _ => Self::Memory(Arc::new(RwLock::new(HashMap::new()))),
        }
    }

    pub async fn insert(&self, account: AccountRecord) {
        match self {
            Self::Memory(store) => {
                store.write().await.insert(account.id, account);
            }
            Self::Postgres(pool) => {
                sqlx::query("insert into identity_accounts (id, username, device_id) values ($1, $2, $3)")
                    .bind(account.id)
                    .bind(account.username)
                    .bind(account.device_id)
                    .execute(pool)
                    .await
                    .unwrap();
            }
        }
    }

    pub async fn find_by_username(&self, username: &str) -> Option<AccountRecord> {
        match self {
            Self::Memory(store) => store
                .read()
                .await
                .values()
                .find(|record| record.username == username)
                .cloned(),
            Self::Postgres(pool) => sqlx::query(
                "select id, username, device_id from identity_accounts where username = $1",
            )
            .bind(username)
            .fetch_optional(pool)
            .await
            .unwrap()
            .map(|row| AccountRecord {
                id: row.get("id"),
                username: row.get("username"),
                device_id: row.get("device_id"),
            }),
        }
    }

    pub async fn find_by_id(&self, account_id: Uuid) -> Option<AccountRecord> {
        match self {
            Self::Memory(store) => store.read().await.get(&account_id).cloned(),
            Self::Postgres(pool) => sqlx::query(
                "select id, username, device_id from identity_accounts where id = $1",
            )
            .bind(account_id)
            .fetch_optional(pool)
            .await
            .unwrap()
            .map(|row| AccountRecord {
                id: row.get("id"),
                username: row.get("username"),
                device_id: row.get("device_id"),
            }),
        }
    }

    pub async fn find_by_device_id(&self, device_id: &str) -> Option<AccountRecord> {
        match self {
            Self::Memory(store) => store
                .read()
                .await
                .values()
                .find(|record| record.device_id == device_id)
                .cloned(),
            Self::Postgres(pool) => sqlx::query(
                "select id, username, device_id from identity_accounts where device_id = $1",
            )
            .bind(device_id)
            .fetch_optional(pool)
            .await
            .unwrap()
            .map(|row| AccountRecord {
                id: row.get("id"),
                username: row.get("username"),
                device_id: row.get("device_id"),
            }),
        }
    }
}