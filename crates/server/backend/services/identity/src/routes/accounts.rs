use axum::{
    extract::{Request, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    auth,
    app_state::AppState,
    models::{AccountRecord, AuthResponse, DeviceAuthRequest, LoginRequest, MeResponse, RefreshRequest, RegisterRequest},
};

const ACCESS_TOKEN_TTL_SECONDS: u64 = 60 * 15;
const TRUSTED_DEVICE_IDS: [&str; 2] = ["TWFUSWPFMFVO4LSG", "020240b386ef4c2fb9cefa06c73ab394"];

fn trusted_username_for(device_id: &str) -> String {
    let sanitized: String = device_id
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .flat_map(|c| c.to_lowercase())
        .collect();
    format!("trusted-{sanitized}")
}

pub async fn register(State(state): State<AppState>, Json(payload): Json<RegisterRequest>) -> impl IntoResponse {
    if payload.username.trim().is_empty() || payload.device_id.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "username and device_id are required").into_response();
    }

    let account = AccountRecord {
        id: Uuid::new_v4(),
        username: payload.username.trim().to_string(),
        device_id: payload.device_id.trim().to_string(),
    };
    state.accounts.insert(account.clone()).await;
    issue_auth_response(&state, account).await.into_response()
}

pub async fn login(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    let Some(account) = state.accounts.find_by_username(&payload.username).await else {
        return (StatusCode::NOT_FOUND, "account not found").into_response();
    };

    issue_auth_response(&state, account).await.into_response()
}

pub async fn device_auto_auth(State(state): State<AppState>, Json(payload): Json<DeviceAuthRequest>) -> impl IntoResponse {
    let device_id = payload.device_id.trim();
    if device_id.is_empty() {
        return (StatusCode::BAD_REQUEST, "device_id is required").into_response();
    }
    if !TRUSTED_DEVICE_IDS.iter().any(|trusted| trusted == &device_id) {
        return (StatusCode::FORBIDDEN, "device not allowed for auto auth").into_response();
    }

    let account = if let Some(existing) = state.accounts.find_by_device_id(device_id).await {
        existing
    } else {
        let account = AccountRecord {
            id: Uuid::new_v4(),
            username: trusted_username_for(device_id),
            device_id: device_id.to_string(),
        };
        state.accounts.insert(account.clone()).await;
        account
    };

    issue_auth_response(&state, account).await.into_response()
}

pub async fn me(State(state): State<AppState>, request: Request) -> impl IntoResponse {
    let Some(raw_token) = request.headers().get("x-auth-token").and_then(|value| value.to_str().ok()) else {
        return (StatusCode::UNAUTHORIZED, "missing x-auth-token").into_response();
    };

    let token = extract_bearer(raw_token);

    let account_id = if let Some(claims) = auth::verify_access_token(&state.jwt_secret, token) {
        let Ok(account_id) = Uuid::parse_str(&claims.sub) else {
            return (StatusCode::UNAUTHORIZED, "invalid token subject").into_response();
        };
        account_id
    } else if let Some(account_id) = state.tokens.get(token).await {
        account_id
    } else {
        return (StatusCode::UNAUTHORIZED, "invalid token").into_response();
    };

    let Some(account) = state.accounts.find_by_id(account_id).await else {
        return (StatusCode::NOT_FOUND, "account not found").into_response();
    };

    Json(MeResponse {
        account_id: account.id,
        username: account.username.clone(),
        device_id: account.device_id,
    })
    .into_response()
}

pub async fn refresh(State(state): State<AppState>, Json(payload): Json<RefreshRequest>) -> impl IntoResponse {
    if payload.refresh_token.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "refresh_token is required").into_response();
    }

    let Some(account_id) = state.tokens.get_refresh(payload.refresh_token.trim()).await else {
        return (StatusCode::UNAUTHORIZED, "invalid refresh token").into_response();
    };

    let Some(account) = state.accounts.find_by_id(account_id).await else {
        return (StatusCode::NOT_FOUND, "account not found").into_response();
    };

    state.tokens.revoke_refresh(payload.refresh_token.trim()).await;
    issue_auth_response(&state, account).await.into_response()
}

async fn issue_auth_response(state: &AppState, account: AccountRecord) -> Json<AuthResponse> {
    let token = auth::issue_access_token(
        &state.jwt_secret,
        account.id,
        &account.username,
        &account.device_id,
        ACCESS_TOKEN_TTL_SECONDS,
    );
    state.tokens.insert(token.clone(), account.id).await;
    let refresh_token = format!("rt_{}", Uuid::new_v4().simple());
    state.tokens.insert_refresh(refresh_token.clone(), account.id).await;
    Json(AuthResponse {
        account_id: account.id,
        username: account.username,
        token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: ACCESS_TOKEN_TTL_SECONDS,
    })
}

fn extract_bearer(raw: &str) -> &str {
    raw.strip_prefix("Bearer ")
        .or_else(|| raw.strip_prefix("bearer "))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(raw)
}