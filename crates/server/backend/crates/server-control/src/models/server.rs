use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{state::ServerState, visibility::Visibility};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSummary {
    pub id: Uuid,
    pub owner_account_id: Uuid,
    pub link_code: String,
    pub join_code: String,
    pub access_code: Option<String>,
    pub realtime_endpoint: String,
    pub name: String,
    pub region: String,
    pub mode: String,
    pub visibility: Visibility,
    pub max_players: u16,
    pub state: ServerState,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessCheckResponse {
    pub allowed: bool,
    pub reason: String,
}