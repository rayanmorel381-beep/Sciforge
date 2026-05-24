use std::{collections::HashMap, sync::Arc};

use redis::AsyncCommands;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub enum TokenStore {
    Memory(Arc<RwLock<HashMap<String, Uuid>>>),
    Redis(redis::Client),
}

impl TokenStore {
    pub async fn from_env() -> Self {
        match std::env::var("REDIS_URL") {
            Ok(url) if !url.trim().is_empty() => Self::Redis(redis::Client::open(url).unwrap()),
            _ => Self::Memory(Arc::new(RwLock::new(HashMap::new()))),
        }
    }

    pub async fn insert(&self, token: String, account_id: Uuid) {
        match self {
            Self::Memory(store) => {
                store.write().await.insert(token, account_id);
            }
            Self::Redis(client) => {
                let mut connection = client.get_multiplexed_async_connection().await.unwrap();
                let _: () = connection
                    .set_ex(format!("identity:token:{token}"), account_id.to_string(), 60 * 60 * 24)
                    .await
                    .unwrap();
            }
        }
    }

    pub async fn get(&self, token: &str) -> Option<Uuid> {
        match self {
            Self::Memory(store) => store.read().await.get(token).copied(),
            Self::Redis(client) => {
                let mut connection = client.get_multiplexed_async_connection().await.unwrap();
                let value: Option<String> = connection.get(format!("identity:token:{token}")).await.unwrap();
                value.and_then(|raw| Uuid::parse_str(&raw).ok())
            }
        }
    }

    pub async fn insert_refresh(&self, token: String, account_id: Uuid) {
        match self {
            Self::Memory(store) => {
                store.write().await.insert(format!("refresh:{token}"), account_id);
            }
            Self::Redis(client) => {
                let mut connection = client.get_multiplexed_async_connection().await.unwrap();
                let _: () = connection
                    .set_ex(
                        format!("identity:refresh:{token}"),
                        account_id.to_string(),
                        60 * 60 * 24 * 7,
                    )
                    .await
                    .unwrap();
            }
        }
    }

    pub async fn get_refresh(&self, token: &str) -> Option<Uuid> {
        match self {
            Self::Memory(store) => store.read().await.get(&format!("refresh:{token}")).copied(),
            Self::Redis(client) => {
                let mut connection = client.get_multiplexed_async_connection().await.unwrap();
                let value: Option<String> = connection
                    .get(format!("identity:refresh:{token}"))
                    .await
                    .unwrap();
                value.and_then(|raw| Uuid::parse_str(&raw).ok())
            }
        }
    }

    pub async fn revoke_refresh(&self, token: &str) {
        match self {
            Self::Memory(store) => {
                store.write().await.remove(&format!("refresh:{token}"));
            }
            Self::Redis(client) => {
                let mut connection = client.get_multiplexed_async_connection().await.unwrap();
                let _: () = connection.del(format!("identity:refresh:{token}")).await.unwrap();
            }
        }
    }
}