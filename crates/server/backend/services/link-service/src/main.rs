mod app_state;
mod record;
mod routes;
mod service;
mod storage;

use std::net::SocketAddr;

use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post},
};
use core_domain::correlation_middleware;

use app_state::AppState;
use routes::{health, link_codes};

#[tokio::main]
async fn main() {
    let _ = tracing_subscriber::fmt::try_init();

    let port = std::env::var("LINK_SERVICE_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8082);

    let app = Router::new()
        .route("/health", get(health::health))
        .route("/link-codes", post(link_codes::issue_code))
        .route("/link-codes/claim", post(link_codes::claim_code))
        .route("/link-codes/:code", get(link_codes::code_status))
        .layer(from_fn(correlation_middleware))
        .with_state(AppState::new().await);

    let addr = format!("0.0.0.0:{port}").parse::<SocketAddr>().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
