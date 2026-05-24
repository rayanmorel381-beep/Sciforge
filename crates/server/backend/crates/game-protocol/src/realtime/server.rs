use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::realtime::{player::PlayerPresence, session::SessionPhase};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerEvent {
    Welcome {
        server_id: Uuid,
        player: PlayerPresence,
    },
    PresenceSnapshot {
        server_id: Uuid,
        players: Vec<PlayerPresence>,
    },
    PlayerJoined {
        server_id: Uuid,
        player: PlayerPresence,
    },
    PlayerLeft {
        server_id: Uuid,
        player_id: Uuid,
    },
    ActionBroadcast {
        server_id: Uuid,
        player_id: Uuid,
        kind: String,
        payload: Value,
        sequence: u64,
    },
    Pong {
        server_id: Uuid,
        client_time: DateTime<Utc>,
        server_time: DateTime<Utc>,
    },
    Heartbeat {
        server_id: Uuid,
        server_time: DateTime<Utc>,
        sequence: u64,
    },
    PhaseChanged {
        server_id: Uuid,
        phase: SessionPhase,
    },
    Error {
        code: String,
        message: String,
    },
}
