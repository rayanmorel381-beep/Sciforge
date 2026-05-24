use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use link_protocol::{ClaimLinkCodeRequest, IssueLinkCodeRequest};

use crate::{app_state::AppState, service};

pub async fn issue_code(State(state): State<AppState>, Json(payload): Json<IssueLinkCodeRequest>) -> impl IntoResponse {
    if payload.apk_session_id.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "apk_session_id is required").into_response();
    }

    let (code, record, response) = service::create_record(payload);
    state.bindings.insert(code, record).await;
    Json(response).into_response()
}

pub async fn claim_code(State(state): State<AppState>, Json(payload): Json<ClaimLinkCodeRequest>) -> impl IntoResponse {
    let Some(current) = state.bindings.get(&payload.code).await else {
        return (StatusCode::NOT_FOUND, "link code not found").into_response();
    };

    if current.expires_at < Utc::now() {
        return (StatusCode::GONE, "link code expired").into_response();
    }

    let Some(record) = state
        .bindings
        .claim(&payload.code, payload.browser_session_id, payload.account_id)
        .await
    else {
        return (StatusCode::NOT_FOUND, "link code not found").into_response();
    };

    Json(service::to_status_response(&record)).into_response()
}

pub async fn code_status(State(state): State<AppState>, Path(code): Path<String>) -> impl IntoResponse {
    let Some(record) = state.bindings.get(&code).await else {
        return (StatusCode::NOT_FOUND, "link code not found").into_response();
    };

    Json(service::to_status_response(&record)).into_response()
}