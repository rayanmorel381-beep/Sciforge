mod app_state;
mod models;
mod proxy;
mod routes;

use std::net::SocketAddr;

use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post},
};
use core_domain::correlation_middleware;

use app_state::AppState;
use routes::{auth, gameplay, health, link, lobby, servers};

#[tokio::main]
async fn main() {
    let _ = tracing_subscriber::fmt::try_init();

    let port = std::env::var("API_GATEWAY_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8080);

    let app = Router::new()
        .route("/health", get(health::health))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/device-auto", post(auth::device_auto))
        .route("/api/auth/refresh", post(auth::refresh))
        .route("/api/auth/me", get(auth::me))
        .route("/api/link/codes", post(link::issue_code))
        .route("/api/link/codes/claim", post(link::claim_code))
        .route("/api/link/codes/:code", get(link::link_status))
        .route("/api/gameplay", get(gameplay::list_docs))
        .route("/api/gameplay/:slug", get(gameplay::get_doc))
        .route("/api/servers", post(servers::create_server).get(servers::list_servers))
        .route("/api/servers/:id", get(servers::get_server).patch(servers::update_server))
        .route("/api/lobby/public", get(lobby::lobby_public))
        .route("/api/lobby/owned/:account_id", get(lobby::lobby_owned))
        .layer(from_fn(correlation_middleware))
        .with_state(AppState::from_env());

    let addr = format!("0.0.0.0:{port}").parse::<SocketAddr>().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
