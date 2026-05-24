use axum::{extract::{Path, State}, response::IntoResponse};
use uuid::Uuid;

use crate::{app_state::AppState, service};

pub async fn list_public(State(state): State<AppState>) -> impl IntoResponse {
    let url = format!("{}/servers?visibility=public", state.server_manager_url);
    service::fetch_servers(&state.client, &url).await
}

pub async fn list_owned(State(state): State<AppState>, Path(account_id): Path<Uuid>) -> impl IntoResponse {
    let url = format!("{}/servers?owner_account_id={}", state.server_manager_url, account_id);
    service::fetch_servers(&state.client, &url).await
}