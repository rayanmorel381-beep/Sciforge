use axum::http::StatusCode;
use server_control::AccessCheckResponse;
use uuid::Uuid;

pub async fn check_server_access(
    client: &reqwest::Client,
    server_manager_url: &str,
    server_id: Uuid,
    account_id: Uuid,
    invite_code: Option<&str>,
) -> Result<AccessCheckResponse, StatusCode> {
    let mut url = format!("{server_manager_url}/servers/{server_id}/access?account_id={account_id}");
    if let Some(code) = invite_code
        && !code.trim().is_empty()
    {
        url = format!("{url}&invite_code={}", code.trim());
    }

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    if !response.status().is_success() {
        return Err(StatusCode::BAD_GATEWAY);
    }

    response
        .json::<AccessCheckResponse>()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)
}
