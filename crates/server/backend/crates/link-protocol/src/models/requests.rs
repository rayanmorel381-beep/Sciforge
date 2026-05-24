use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueLinkCodeRequest {
    pub apk_session_id: String,
    pub account_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimLinkCodeRequest {
    pub code: String,
    pub browser_session_id: String,
    pub account_id: Option<Uuid>,
}