use axum::http::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct IdentityMeResponse {
    pub account_id: Uuid,
    pub username: String,
    pub device_id: String,
}

pub async fn authenticate(
    client: &reqwest::Client,
    identity_url: &str,
    token: &str,
) -> Result<IdentityMeResponse, StatusCode> {
    let response = client
        .get(format!("{identity_url}/accounts/me"))
        .header("x-auth-token", token)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    if response.status() == StatusCode::UNAUTHORIZED {
        return Err(StatusCode::UNAUTHORIZED);
    }

    if !response.status().is_success() {
        return Err(StatusCode::BAD_GATEWAY);
    }

    response
        .json::<IdentityMeResponse>()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)
}
