use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct LinkRecord {
    pub binding_id: Uuid,
    pub code: String,
    pub apk_session_id: String,
    pub browser_session_id: Option<String>,
    pub account_id: Option<Uuid>,
    pub expires_at: DateTime<Utc>,
}