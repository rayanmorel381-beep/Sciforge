use axum::{http::StatusCode, response::IntoResponse, Json};
use server_control::ServerSummary;

pub async fn fetch_servers(client: &reqwest::Client, url: &str) -> axum::response::Response {
    let response = match client.get(url).send().await {
        Ok(response) => response,
        Err(_) => return (StatusCode::BAD_GATEWAY, "server-manager unavailable").into_response(),
    };

    let servers = match response.json::<Vec<ServerSummary>>().await {
        Ok(servers) => servers,
        Err(_) => return (StatusCode::BAD_GATEWAY, "invalid server-manager response").into_response(),
    };

    Json(servers).into_response()
}