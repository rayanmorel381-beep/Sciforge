use std::sync::atomic::Ordering;

use futures_util::StreamExt;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app_state::AppState;

#[derive(Debug, Serialize, Deserialize)]
struct ClusterEnvelope {
    source_instance_id: String,
    payload: String,
}

pub async fn publish(state: &AppState, server_id: Uuid, payload: &str) {
    let Some(client) = &state.redis_client else {
        return;
    };

    let envelope = ClusterEnvelope {
        source_instance_id: state.instance_id.to_string(),
        payload: payload.to_string(),
    };
    let Ok(json_payload) = serde_json::to_string(&envelope) else {
        return;
    };

    let channel = channel_name(server_id);
    if let Ok(mut connection) = client.get_multiplexed_async_connection().await {
        let result: redis::RedisResult<()> = connection.publish(channel, json_payload).await;
        if result.is_ok() {
            state
                .metrics
                .cluster_messages_out
                .fetch_add(1, Ordering::Relaxed);
        }
    }
}

pub fn spawn_subscriber(state: AppState) {
    let Some(client) = state.redis_client.clone() else {
        return;
    };

    tokio::spawn(async move {
        let Ok(mut pubsub) = client.get_async_pubsub().await else {
            return;
        };
        if pubsub.psubscribe("realtime:server:*").await.is_err() {
            return;
        }

        let mut stream = pubsub.on_message();
        while let Some(message) = stream.next().await {
            let channel = message.get_channel_name().to_string();
            let Some(server_id) = parse_server_id(&channel) else {
                continue;
            };
            let Ok(raw_payload) = message.get_payload::<String>() else {
                continue;
            };
            let Ok(envelope) = serde_json::from_str::<ClusterEnvelope>(&raw_payload) else {
                continue;
            };
            if envelope.source_instance_id == state.instance_id.to_string() {
                continue;
            }

            let room = {
                let mut registry = state.rooms.write().await;
                registry.get_or_create_room(server_id)
            };
            let sender = {
                let guard = room.read().await;
                guard.sender()
            };
            if sender.send(envelope.payload).is_ok() {
                state
                    .metrics
                    .cluster_messages_in
                    .fetch_add(1, Ordering::Relaxed);
            }
        }
    });
}

fn channel_name(server_id: Uuid) -> String {
    format!("realtime:server:{server_id}")
}

fn parse_server_id(channel: &str) -> Option<Uuid> {
    channel
        .strip_prefix("realtime:server:")
        .and_then(|value| Uuid::parse_str(value).ok())
}
