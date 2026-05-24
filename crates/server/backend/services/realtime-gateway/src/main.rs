mod app_state;
mod auth;
mod metrics;
mod protocol;
mod rooms;
mod routes;

use std::net::SocketAddr;

use axum::{
    Router,
    middleware::from_fn,
    routing::get,
};
use core_domain::correlation_middleware;

use app_state::AppState;
use rooms::distributed;
use routes::{health, realtime};

#[tokio::main]
async fn main() {
    let _ = tracing_subscriber::fmt::try_init();

    let port = std::env::var("REALTIME_GATEWAY_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8085);

    let state = AppState::new();
    distributed::spawn_subscriber(state.clone());

    let app = Router::new()
        .route("/health", get(health::health))
        .route("/metrics", get(health::metrics))
        .route("/realtime/servers/:server_id/events", get(realtime::events_ws))
        .layer(from_fn(correlation_middleware))
        .with_state(state);

    let addr = format!("0.0.0.0:{port}").parse::<SocketAddr>().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
