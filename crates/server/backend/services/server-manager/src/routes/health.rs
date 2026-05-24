use axum::Json;
use core_domain::{ServiceHealth, service_health};

pub async fn health() -> Json<ServiceHealth> {
    Json(service_health("server-manager"))
}