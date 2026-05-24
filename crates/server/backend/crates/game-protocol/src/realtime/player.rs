use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPresence {
    pub player_id: Uuid,
    pub connected_at: DateTime<Utc>,
}
