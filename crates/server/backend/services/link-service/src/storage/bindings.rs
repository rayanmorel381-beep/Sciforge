use std::{collections::HashMap, sync::Arc};

use sqlx::{Row, postgres::PgPoolOptions};
use tokio::sync::RwLock;

use crate::record::LinkRecord;

#[derive(Clone)]
pub enum BindingStore {
    Memory(Arc<RwLock<HashMap<String, LinkRecord>>>),
    Postgres(sqlx::PgPool),
}

impl BindingStore {
    pub async fn from_env() -> Self {
        match std::env::var("LINK_SERVICE_DATABASE_URL") {
            Ok(url) if !url.trim().is_empty() => {
                let pool = PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&url)
                    .await
                    .unwrap();
                sqlx::query(
                    "create table if not exists link_bindings (binding_id uuid primary key, code text not null unique, apk_session_id text not null, browser_session_id text, account_id uuid, expires_at timestamptz not null)",
                )
                .execute(&pool)
                .await
                .unwrap();
                Self::Postgres(pool)
            }
            _ => Self::Memory(Arc::new(RwLock::new(HashMap::new()))),
        }
    }

    pub async fn insert(&self, code: String, record: LinkRecord) {
        match self {
            Self::Memory(store) => {
                store.write().await.insert(code, record);
            }
            Self::Postgres(pool) => {
                sqlx::query(
                    "insert into link_bindings (binding_id, code, apk_session_id, browser_session_id, account_id, expires_at) values ($1, $2, $3, $4, $5, $6)",
                )
                .bind(record.binding_id)
                .bind(record.code)
                .bind(record.apk_session_id)
                .bind(record.browser_session_id)
                .bind(record.account_id)
                .bind(record.expires_at)
                .execute(pool)
                .await
                .unwrap();
            }
        }
    }

    pub async fn get(&self, code: &str) -> Option<LinkRecord> {
        match self {
            Self::Memory(store) => store.read().await.get(code).cloned(),
            Self::Postgres(pool) => sqlx::query(
                "select binding_id, code, apk_session_id, browser_session_id, account_id, expires_at from link_bindings where code = $1",
            )
            .bind(code)
            .fetch_optional(pool)
            .await
            .unwrap()
            .map(map_record),
        }
    }

    pub async fn claim(&self, code: &str, browser_session_id: String, account_id: Option<uuid::Uuid>) -> Option<LinkRecord> {
        match self {
            Self::Memory(store) => {
                let mut guard = store.write().await;
                let record = guard.get_mut(code)?;
                record.browser_session_id = Some(browser_session_id);
                if record.account_id.is_none() {
                    record.account_id = account_id;
                }
                Some(record.clone())
            }
            Self::Postgres(pool) => {
                let current = self.get(code).await?;
                let next_account_id = current.account_id.or(account_id);
                sqlx::query(
                    "update link_bindings set browser_session_id = $2, account_id = $3 where code = $1",
                )
                .bind(code)
                .bind(browser_session_id)
                .bind(next_account_id)
                .execute(pool)
                .await
                .unwrap();
                self.get(code).await
            }
        }
    }
}

fn map_record(row: sqlx::postgres::PgRow) -> LinkRecord {
    LinkRecord {
        binding_id: row.get("binding_id"),
        code: row.get("code"),
        apk_session_id: row.get("apk_session_id"),
        browser_session_id: row.get("browser_session_id"),
        account_id: row.get("account_id"),
        expires_at: row.get("expires_at"),
    }
}