use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Value;
use server_control::{CreateServerRequest, UpdateServerRequest};
use uuid::Uuid;

use crate::{app_state::AppState, models::ServerListQuery, proxy};

async fn authenticated_account_id(state: &AppState, headers: &HeaderMap) -> Result<String, Response> {
    let Some(token_header) = headers.get("x-auth-token") else {
        return Err((StatusCode::UNAUTHORIZED, "missing x-auth-token").into_response());
    };

    let Ok(token) = token_header.to_str() else {
        return Err((StatusCode::UNAUTHORIZED, "invalid x-auth-token").into_response());
    };

    let me_response = match state
        .client
        .get(format!("{}/accounts/me", state.identity_url))
        .header("x-auth-token", token)
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return Err((StatusCode::BAD_GATEWAY, "identity unavailable").into_response()),
    };

    if !me_response.status().is_success() {
        return Err((StatusCode::UNAUTHORIZED, "invalid session").into_response());
    }

    let me_payload = match me_response.json::<Value>().await {
        Ok(payload) => payload,
        Err(_) => return Err((StatusCode::BAD_GATEWAY, "identity invalid json").into_response()),
    };

    let Some(account_id) = me_payload.get("account_id").and_then(Value::as_str) else {
        return Err((StatusCode::BAD_GATEWAY, "identity missing account").into_response());
    };

    Ok(account_id.to_string())
}

async fn fetch_server(state: &AppState, id: Uuid) -> Result<Value, Response> {
    let response = match state
        .client
        .get(format!("{}/servers/{}", state.server_manager_url, id))
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return Err((StatusCode::BAD_GATEWAY, "upstream unavailable").into_response()),
    };

    let status = response.status();
    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return Err((StatusCode::BAD_GATEWAY, "upstream invalid body").into_response()),
    };

    if !status.is_success() {
        let mapped = StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
        return Err((mapped, body).into_response());
    }

    match serde_json::from_str::<Value>(&body) {
        Ok(server) => Ok(server),
        Err(_) => Err((StatusCode::BAD_GATEWAY, "upstream invalid json").into_response()),
    }
}

pub async fn create_server(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateServerRequest>,
) -> impl IntoResponse {
    let authenticated_account_id = match authenticated_account_id(&state, &headers).await {
        Ok(account_id) => account_id,
        Err(response) => return response,
    };

    if authenticated_account_id != payload.owner_account_id.to_string() {
        return (StatusCode::FORBIDDEN, "owner_account_id mismatch").into_response();
    }

    proxy::proxy_post::<CreateServerRequest, Value>(&state.client, &format!("{}/servers", state.server_manager_url), &payload).await
}

pub async fn list_servers(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<ServerListQuery>,
) -> impl IntoResponse {
    let mut url = format!("{}/servers", state.server_manager_url);
    let mut parts = Vec::new();

    if let Some(account_id) = query.owner_account_id {
        let authenticated_account_id = match authenticated_account_id(&state, &headers).await {
            Ok(account_id) => account_id,
            Err(response) => return response,
        };
        if authenticated_account_id != account_id.to_string() {
            return (StatusCode::FORBIDDEN, "owner_account_id mismatch").into_response();
        }
        parts.push(format!("owner_account_id={account_id}"));
    }

    if let Some(visibility) = query.visibility {
        if visibility != "public" && query.owner_account_id.is_none() {
            return (StatusCode::FORBIDDEN, "non-public visibility requires owner filter").into_response();
        }
        parts.push(format!("visibility={visibility}"));
    } else if query.owner_account_id.is_none() {
        parts.push("visibility=public".to_string());
    }

    if !parts.is_empty() {
        url = format!("{url}?{}", parts.join("&"));
    }
    proxy::proxy_get::<Value>(&state.client, &url).await
}

pub async fn get_server(State(state): State<AppState>, headers: HeaderMap, Path(id): Path<Uuid>) -> impl IntoResponse {
    let server = match fetch_server(&state, id).await {
        Ok(server) => server,
        Err(response) => return response,
    };

    let visibility = server.get("visibility").and_then(Value::as_str).unwrap_or("public");
    if visibility == "public" {
        return Json(server).into_response();
    }

    let authenticated_account_id = match authenticated_account_id(&state, &headers).await {
        Ok(account_id) => account_id,
        Err(response) => return response,
    };

    let owner_account_id = server
        .get("owner_account_id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    if owner_account_id != authenticated_account_id {
        return (StatusCode::FORBIDDEN, "server access denied").into_response();
    }

    Json(server).into_response()
}

pub async fn update_server(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateServerRequest>,
) -> impl IntoResponse {
    let authenticated_account_id = match authenticated_account_id(&state, &headers).await {
        Ok(account_id) => account_id,
        Err(response) => return response,
    };

    let server = match fetch_server(&state, id).await {
        Ok(server) => server,
        Err(response) => return response,
    };

    let owner_account_id = server
        .get("owner_account_id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    if owner_account_id != authenticated_account_id {
        return (StatusCode::FORBIDDEN, "server update denied").into_response();
    }

    proxy::proxy_patch::<UpdateServerRequest, Value>(&state.client, &format!("{}/servers/{}", state.server_manager_url, id), &payload).await
}