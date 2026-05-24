use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub device_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceAuthRequest {
    pub device_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerListQuery {
    pub owner_account_id: Option<Uuid>,
    pub visibility: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GatewayHealth {
    pub gateway: core_domain::ServiceHealth,
    pub upstreams: Vec<String>,
    pub upstream_health: Vec<UpstreamHealth>,
}

#[derive(Debug, Serialize)]
pub struct UpstreamHealth {
    pub service: String,
    pub url: String,
    pub status: String,
    pub http_status: Option<u16>,
}