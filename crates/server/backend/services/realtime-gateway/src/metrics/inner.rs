use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Default)]
pub struct RealtimeMetrics {
    pub connections_total: AtomicU64,
    pub active_connections: AtomicU64,
    pub auth_failures: AtomicU64,
    pub messages_in: AtomicU64,
    pub messages_out: AtomicU64,
    pub cluster_messages_in: AtomicU64,
    pub cluster_messages_out: AtomicU64,
    pub dropped_actions: AtomicU64,
}

impl RealtimeMetrics {
    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            connections_total: self.connections_total.load(Ordering::Relaxed),
            active_connections: self.active_connections.load(Ordering::Relaxed),
            auth_failures: self.auth_failures.load(Ordering::Relaxed),
            messages_in: self.messages_in.load(Ordering::Relaxed),
            messages_out: self.messages_out.load(Ordering::Relaxed),
            cluster_messages_in: self.cluster_messages_in.load(Ordering::Relaxed),
            cluster_messages_out: self.cluster_messages_out.load(Ordering::Relaxed),
            dropped_actions: self.dropped_actions.load(Ordering::Relaxed),
        }
    }
}

#[derive(serde::Serialize)]
pub struct MetricsSnapshot {
    pub connections_total: u64,
    pub active_connections: u64,
    pub auth_failures: u64,
    pub messages_in: u64,
    pub messages_out: u64,
    pub cluster_messages_in: u64,
    pub cluster_messages_out: u64,
    pub dropped_actions: u64,
}
