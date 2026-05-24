use axum::{extract::{Path, State}, response::IntoResponse};
use serde_json::Value;
use uuid::Uuid;

use crate::{app_state::AppState, proxy};

pub async fn lobby_public(State(state): State<AppState>) -> impl IntoResponse {
    proxy::proxy_get::<Value>(&state.client, &format!("{}/lobby/public", state.lobby_url)).await
}

pub async fn lobby_owned(State(state): State<AppState>, Path(account_id): Path<Uuid>) -> impl IntoResponse {
    proxy::proxy_get::<Value>(&state.client, &format!("{}/lobby/owned/{}", state.lobby_url, account_id)).await
}