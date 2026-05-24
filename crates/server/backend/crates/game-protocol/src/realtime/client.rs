use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientEvent {
    Join {
        display_name: Option<String>,
    },
    Action {
        kind: String,
        payload: Value,
    },
    Ping {
        client_time: DateTime<Utc>,
    },
    Leave,
}
