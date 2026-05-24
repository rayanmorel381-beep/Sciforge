use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone)]
pub struct AccountRecord {
    pub id: Uuid,
    pub username: String,
    pub device_id: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub device_id: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct DeviceAuthRequest {
    pub device_id: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub account_id: Uuid,
    pub username: String,
    pub token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub account_id: Uuid,
    pub username: String,
    pub device_id: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}