use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: String,
    pub username: String,
    pub device_id: String,
    pub exp: u64,
    pub iat: u64,
}

pub fn issue_access_token(secret: &str, account_id: Uuid, username: &str, device_id: &str, ttl_seconds: u64) -> String {
    let iat = now_unix();
    let claims = AccessClaims {
        sub: account_id.to_string(),
        username: username.to_string(),
        device_id: device_id.to_string(),
        exp: iat.saturating_add(ttl_seconds),
        iat,
    };
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

pub fn verify_access_token(secret: &str, token: &str) -> Option<AccessClaims> {
    decode::<AccessClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .ok()
    .map(|data| data.claims)
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
