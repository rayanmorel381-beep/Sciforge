use crate::storage::ServerStore;

#[derive(Clone)]
pub struct AppState {
    pub servers: ServerStore,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            servers: ServerStore::from_env().await,
        }
    }
}