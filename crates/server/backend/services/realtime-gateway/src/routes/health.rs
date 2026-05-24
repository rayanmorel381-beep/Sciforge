use axum::{extract::State, Json};
use core_domain::{service_health, ServiceHealth};

use crate::{app_state::AppState, metrics::MetricsSnapshot};

pub async fn health() -> Json<ServiceHealth> {
    Json(service_health("realtime-gateway"))
}

pub async fn metrics(State(state): State<AppState>) -> Json<MetricsSnapshot> {
    Json(state.metrics.snapshot())
}
