use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{state::ServerState, visibility::Visibility};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServerRequest {
    pub owner_account_id: Uuid,
    pub link_code: String,
    pub name: String,
    pub region: String,
    pub mode: String,
    pub visibility: Visibility,
    pub max_players: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateServerRequest {
    pub visibility: Option<Visibility>,
    pub state: Option<ServerState>,
    pub max_players: Option<u16>,
}