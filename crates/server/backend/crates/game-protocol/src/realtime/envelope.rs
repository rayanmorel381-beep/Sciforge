use serde::{Deserialize, Serialize};

use crate::realtime::{ClientEvent, ServerEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientEnvelope {
    pub event: ClientEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerEnvelope {
    pub event: ServerEvent,
}
