use axum::{extract::State, response::IntoResponse, Json};
use serde_json::Value;

use crate::{
    app_state::AppState,
    models::{DeviceAuthRequest, LoginRequest, RefreshRequest, RegisterRequest},
    proxy,
};

pub async fn register(State(state): State<AppState>, Json(payload): Json<RegisterRequest>) -> impl IntoResponse {
    proxy::proxy_post::<RegisterRequest, Value>(&state.client, &format!("{}/accounts/register", state.identity_url), &payload).await
}

pub async fn login(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    proxy::proxy_post::<LoginRequest, Value>(&state.client, &format!("{}/accounts/login", state.identity_url), &payload).await
}

pub async fn device_auto(State(state): State<AppState>, Json(payload): Json<DeviceAuthRequest>) -> impl IntoResponse {
    proxy::proxy_post::<DeviceAuthRequest, Value>(&state.client, &format!("{}/accounts/device-auto", state.identity_url), &payload).await
}

pub async fn refresh(State(state): State<AppState>, Json(payload): Json<RefreshRequest>) -> impl IntoResponse {
    proxy::proxy_post::<RefreshRequest, Value>(&state.client, &format!("{}/accounts/refresh", state.identity_url), &payload)
        .await
}

pub async fn me(State(state): State<AppState>, headers: axum::http::HeaderMap) -> impl IntoResponse {
    let mut request = state.client.get(format!("{}/accounts/me", state.identity_url));
    if let Some(token) = headers.get("x-auth-token") {
        request = request.header("x-auth-token", token.clone());
    }
    proxy::proxy_request::<Value>(request).await
}