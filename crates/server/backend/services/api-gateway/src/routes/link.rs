use axum::{extract::{Path, State}, response::IntoResponse, Json};
use link_protocol::{ClaimLinkCodeRequest, IssueLinkCodeRequest};
use serde_json::Value;

use crate::{app_state::AppState, proxy};

pub async fn issue_code(State(state): State<AppState>, Json(payload): Json<IssueLinkCodeRequest>) -> impl IntoResponse {
    proxy::proxy_post::<IssueLinkCodeRequest, Value>(&state.client, &format!("{}/link-codes", state.link_service_url), &payload).await
}

pub async fn claim_code(State(state): State<AppState>, Json(payload): Json<ClaimLinkCodeRequest>) -> impl IntoResponse {
    proxy::proxy_post::<ClaimLinkCodeRequest, Value>(&state.client, &format!("{}/link-codes/claim", state.link_service_url), &payload).await
}

pub async fn link_status(State(state): State<AppState>, Path(code): Path<String>) -> impl IntoResponse {
    proxy::proxy_get::<Value>(&state.client, &format!("{}/link-codes/{}", state.link_service_url, code)).await
}