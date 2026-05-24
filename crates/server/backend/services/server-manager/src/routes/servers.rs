use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{app_state::AppState, models::{AccessQuery, ListServersQuery}, service};
use server_control::{AccessCheckResponse, CreateServerRequest, ServerState, UpdateServerRequest, Visibility};

pub async fn create_server(State(state): State<AppState>, Json(payload): Json<CreateServerRequest>) -> impl IntoResponse {
    if payload.name.trim().is_empty() || payload.region.trim().is_empty() || payload.mode.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "name, region and mode are required").into_response();
    }

    let server = service::build_server(payload);
    state.servers.insert(server.clone()).await;
    Json(server).into_response()
}

pub async fn list_servers(State(state): State<AppState>, Query(query): Query<ListServersQuery>) -> impl IntoResponse {
    Json(state.servers.list(query.owner_account_id, query.visibility).await)
}

pub async fn get_server(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let Some(server) = state.servers.get(id).await else {
        return (StatusCode::NOT_FOUND, "server not found").into_response();
    };
    Json(server).into_response()
}

pub async fn update_server(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateServerRequest>,
) -> impl IntoResponse {
    let Some(server) = state.servers.update(id, payload).await else {
        return (StatusCode::NOT_FOUND, "server not found").into_response();
    };

    Json(server).into_response()
}

pub async fn check_access(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(query): Query<AccessQuery>,
) -> impl IntoResponse {
    let Some(server) = state.servers.get(id).await else {
        return (StatusCode::NOT_FOUND, "server not found").into_response();
    };

    if server.state != ServerState::Online {
        return Json(AccessCheckResponse {
            allowed: false,
            reason: "server_not_online".to_string(),
        })
        .into_response();
    }

    let allowed = match server.visibility {
        Visibility::Public | Visibility::Unlisted => true,
        Visibility::Private => {
            if server.owner_account_id == query.account_id {
                true
            } else {
                server
                    .access_code
                    .as_ref()
                    .is_some_and(|code| Some(code) == query.invite_code.as_ref())
            }
        }
        Visibility::InviteOnly => server
            .access_code
            .as_ref()
            .is_some_and(|code| Some(code) == query.invite_code.as_ref()),
    };

    let reason = if allowed { "ok" } else { "access_denied" };

    Json(AccessCheckResponse {
        allowed,
        reason: reason.to_string(),
    })
    .into_response()
}