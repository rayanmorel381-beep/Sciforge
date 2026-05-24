use crate::storage::{AccountStore, TokenStore};

#[derive(Clone)]
pub struct AppState {
    pub accounts: AccountStore,
    pub tokens: TokenStore,
    pub jwt_secret: String,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            accounts: AccountStore::from_env().await,
            tokens: TokenStore::from_env().await,
            jwt_secret: std::env::var("JWT_SECRET").unwrap_or_else(|_| "local-dev-secret-change-me".to_string()),
        }
    }
}