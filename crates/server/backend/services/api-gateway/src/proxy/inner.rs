use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Serialize, de::DeserializeOwned};

pub async fn proxy_get<T: DeserializeOwned + Serialize>(client: &reqwest::Client, url: &str) -> axum::response::Response {
    proxy_request::<T>(client.get(url.to_string())).await
}

pub async fn proxy_post<B: Serialize, T: DeserializeOwned + Serialize>(
    client: &reqwest::Client,
    url: &str,
    payload: &B,
) -> axum::response::Response {
    proxy_request::<T>(client.post(url.to_string()).json(payload)).await
}

pub async fn proxy_patch<B: Serialize, T: DeserializeOwned + Serialize>(
    client: &reqwest::Client,
    url: &str,
    payload: &B,
) -> axum::response::Response {
    proxy_request::<T>(client.patch(url.to_string()).json(payload)).await
}

pub async fn proxy_request<T: DeserializeOwned + Serialize>(request: reqwest::RequestBuilder) -> axum::response::Response {
    let response = match request.send().await {
        Ok(response) => response,
        Err(_) => return (StatusCode::BAD_GATEWAY, "upstream unavailable").into_response(),
    };

    let status = response.status();
    let bytes = match response.bytes().await {
        Ok(bytes) => bytes,
        Err(_) => return (StatusCode::BAD_GATEWAY, "upstream invalid body").into_response(),
    };

    if status.is_success() {
        let payload = match serde_json::from_slice::<T>(&bytes) {
            Ok(payload) => payload,
            Err(_) => return (StatusCode::BAD_GATEWAY, "upstream invalid json").into_response(),
        };
        Json(payload).into_response()
    } else {
        let body = String::from_utf8_lossy(&bytes).to_string();
        let mapped = StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
        (mapped, body).into_response()
    }
}