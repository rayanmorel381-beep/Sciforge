mod app_state;
mod models;
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
use routes::{health, servers};

#[tokio::main]
async fn main() {
    let _ = tracing_subscriber::fmt::try_init();

    let port = std::env::var("SERVER_MANAGER_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8083);

    let app = Router::new()
        .route("/health", get(health::health))
        .route("/servers", post(servers::create_server).get(servers::list_servers))
        .route("/servers/:id", get(servers::get_server).patch(servers::update_server))
        .route("/servers/:id/access", get(servers::check_access))
        .layer(from_fn(correlation_middleware))
        .with_state(AppState::new().await);

    let addr = format!("0.0.0.0:{port}").parse::<SocketAddr>().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
