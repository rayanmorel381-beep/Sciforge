use axum::{extract::State, Json};
use core_domain::service_health;

use crate::{app_state::AppState, models::{GatewayHealth, UpstreamHealth}};

async fn check_upstream(client: &reqwest::Client, service: &str, base_url: &str) -> UpstreamHealth {
    let response = client
        .get(format!("{}/health", base_url.trim_end_matches('/')))
        .timeout(std::time::Duration::from_secs(2))
        .send()
        .await;

    match response {
        Ok(res) if res.status().is_success() => UpstreamHealth {
            service: service.to_string(),
            url: base_url.to_string(),
            status: "ok".to_string(),
            http_status: Some(res.status().as_u16()),
        },
        Ok(res) => UpstreamHealth {
            service: service.to_string(),
            url: base_url.to_string(),
            status: "degraded".to_string(),
            http_status: Some(res.status().as_u16()),
        },
        Err(_) => UpstreamHealth {
            service: service.to_string(),
            url: base_url.to_string(),
            status: "down".to_string(),
            http_status: None,
        },
    }
}

pub async fn health(State(state): State<AppState>) -> Json<GatewayHealth> {
    let identity = check_upstream(&state.client, "identity", &state.identity_url);
    let link_service = check_upstream(&state.client, "link-service", &state.link_service_url);
    let server_manager = check_upstream(&state.client, "server-manager", &state.server_manager_url);
    let lobby = check_upstream(&state.client, "lobby", &state.lobby_url);
    let (identity, link_service, server_manager, lobby) = tokio::join!(identity, link_service, server_manager, lobby);
    let upstream_health = vec![identity, link_service, server_manager, lobby];

    let mut gateway = service_health("api-gateway");
    if upstream_health.iter().any(|u| u.status != "ok") {
        gateway.status = "degraded".to_string();
    }

    Json(GatewayHealth {
        gateway,
        upstreams: vec![
            state.identity_url.clone(),
            state.link_service_url.clone(),
            state.server_manager_url.clone(),
            state.lobby_url.clone(),
        ],
        upstream_health,
    })
}