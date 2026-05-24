use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::status::LinkStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueLinkCodeResponse {
    pub code: String,
    pub expires_at: DateTime<Utc>,
    pub binding_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkStatusResponse {
    pub binding_id: Uuid,
    pub code: String,
    pub status: LinkStatus,
    pub apk_session_id: String,
    pub browser_session_id: Option<String>,
    pub account_id: Option<Uuid>,
    pub expires_at: DateTime<Utc>,
}