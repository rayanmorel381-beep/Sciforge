use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub service: String,
    pub status: String,
    pub now: DateTime<Utc>,
}

pub fn service_health(service: &str) -> ServiceHealth {
    ServiceHealth {
        service: service.to_string(),
        status: "ok".to_string(),
        now: Utc::now(),
    }
}