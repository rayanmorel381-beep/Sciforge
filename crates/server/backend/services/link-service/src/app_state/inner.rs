use crate::storage::BindingStore;

#[derive(Clone)]
pub struct AppState {
    pub bindings: BindingStore,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            bindings: BindingStore::from_env().await,
        }
    }
}