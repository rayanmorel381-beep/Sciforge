mod app_state;
mod auth;
mod models;
mod routes;
mod storage;

use std::net::SocketAddr;

use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post},
};
use core_domain::correlation_middleware;

use app_state::AppState;
use routes::{accounts, health};

#[tokio::main]
async fn main() {
    let _ = tracing_subscriber::fmt::try_init();

    let port = std::env::var("IDENTITY_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8081);

    let app = Router::new()
        .route("/health", get(health::health))
        .route("/accounts/register", post(accounts::register))
        .route("/accounts/login", post(accounts::login))
        .route("/accounts/device-auto", post(accounts::device_auto_auth))
        .route("/accounts/me", get(accounts::me))
        .route("/accounts/refresh", post(accounts::refresh))
        .layer(from_fn(correlation_middleware))
        .with_state(AppState::new().await);

    let addr = format!("0.0.0.0:{port}").parse::<SocketAddr>().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
