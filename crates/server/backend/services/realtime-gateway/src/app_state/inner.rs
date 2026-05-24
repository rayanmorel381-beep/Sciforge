use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{metrics::RealtimeMetrics, rooms::registry::RoomRegistry};

#[derive(Clone)]
pub struct AppState {
    pub client: reqwest::Client,
    pub identity_url: String,
    pub server_manager_url: String,
    pub redis_client: Option<redis::Client>,
    pub instance_id: Uuid,
    pub rooms: Arc<RwLock<RoomRegistry>>,
    pub metrics: Arc<RealtimeMetrics>,
}

impl AppState {
    pub fn new() -> Self {
        let redis_client = std::env::var("REDIS_URL")
            .ok()
            .and_then(|url| if url.trim().is_empty() { None } else { redis::Client::open(url).ok() });

        Self {
            client: reqwest::Client::new(),
            identity_url: std::env::var("IDENTITY_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".to_string()),
            server_manager_url: std::env::var("SERVER_MANAGER_URL").unwrap_or_else(|_| "http://127.0.0.1:8083".to_string()),
            redis_client,
            instance_id: Uuid::new_v4(),
            rooms: Arc::new(RwLock::new(RoomRegistry::new())),
            metrics: Arc::new(RealtimeMetrics::default()),
        }
    }
}
