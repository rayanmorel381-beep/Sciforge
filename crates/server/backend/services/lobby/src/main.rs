mod app_state;
mod routes;
mod service;

use std::net::SocketAddr;

use axum::{
    Router,
    middleware::from_fn,
    routing::get,
};
use core_domain::correlation_middleware;

use app_state::AppState;
use routes::{health, lobby};

#[tokio::main]
async fn main() {
    let _ = tracing_subscriber::fmt::try_init();

    let port = std::env::var("LOBBY_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8084);
    let server_manager_url = std::env::var("SERVER_MANAGER_URL").unwrap_or_else(|_| "http://127.0.0.1:8083".to_string());

    let app = Router::new()
        .route("/health", get(health::health))
        .route("/lobby/public", get(lobby::list_public))
        .route("/lobby/owned/:account_id", get(lobby::list_owned))
        .layer(from_fn(correlation_middleware))
        .with_state(AppState::new(server_manager_url));

    let addr = format!("0.0.0.0:{port}").parse::<SocketAddr>().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
