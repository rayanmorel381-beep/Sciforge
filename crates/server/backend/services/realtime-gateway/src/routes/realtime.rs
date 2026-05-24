use std::sync::atomic::Ordering;
use std::time::{Duration as StdDuration, Instant};

use axum::{
    extract::{
        Path, Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::{IntoResponse, Response},
};
use futures_util::{SinkExt, StreamExt};
use game_protocol::{ClientEnvelope, ClientEvent};
use tokio::select;
use tokio::time::{Duration, interval};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    auth::{access, identity},
    protocol::query::EventsQuery,
    rooms::distributed,
    rooms::runtime::RoomRuntime,
};

pub async fn events_ws(
    State(state): State<AppState>,
    Path(server_id): Path<Uuid>,
    Query(query): Query<EventsQuery>,
    ws: WebSocketUpgrade,
) -> Response {
    if query.token.trim().is_empty() {
        state.metrics.auth_failures.fetch_add(1, Ordering::Relaxed);
        return (StatusCode::UNAUTHORIZED, "missing token").into_response();
    }

    let identity = match identity::authenticate(&state.client, &state.identity_url, &query.token).await {
        Ok(identity) => identity,
        Err(code) => {
            state.metrics.auth_failures.fetch_add(1, Ordering::Relaxed);
            return (code, "authentication failed").into_response();
        }
    };

    let access = match access::check_server_access(
        &state.client,
        &state.server_manager_url,
        server_id,
        identity.account_id,
        query.invite_code.as_deref(),
    )
    .await
    {
        Ok(decision) => decision,
        Err(code) => return (code, "access check failed").into_response(),
    };

    if !access.allowed {
        state.metrics.auth_failures.fetch_add(1, Ordering::Relaxed);
        return (StatusCode::FORBIDDEN, access.reason).into_response();
    }

    let actor_label = format!("{}@{}", identity.username, identity.device_id);
    ws.on_upgrade(move |socket| handle_socket(state, server_id, identity.account_id, actor_label, socket))
        .into_response()
}

async fn handle_socket(state: AppState, server_id: Uuid, player_id: Uuid, actor_label: String, socket: WebSocket) {
    state.metrics.connections_total.fetch_add(1, Ordering::Relaxed);
    state.metrics.active_connections.fetch_add(1, Ordering::Relaxed);

    let room = {
        let mut registry = state.rooms.write().await;
        registry.get_or_create_room(server_id)
    };
    let sender = {
        let guard = room.read().await;
        guard.sender()
    };
    let mut receiver = {
        let guard = room.read().await;
        guard.subscribe()
    };

    let (mut write, mut read) = socket.split();
    let (welcome, snapshot, joined, phase_event, phase_sync) = {
        let mut guard = room.write().await;
        let (welcome, snapshot, joined, phase_event) = guard.join(server_id, player_id);
        let phase_sync = guard.phase_message(server_id);
        (welcome, snapshot, joined, phase_event, phase_sync)
    };

    if write.send(Message::Text(welcome)).await.is_err() {
        state.metrics.active_connections.fetch_sub(1, Ordering::Relaxed);
        return;
    }
    state.metrics.messages_out.fetch_add(1, Ordering::Relaxed);

    if write.send(Message::Text(snapshot)).await.is_err() {
        state.metrics.active_connections.fetch_sub(1, Ordering::Relaxed);
        return;
    }
    state.metrics.messages_out.fetch_add(1, Ordering::Relaxed);

    if write.send(Message::Text(phase_sync)).await.is_err() {
        state.metrics.active_connections.fetch_sub(1, Ordering::Relaxed);
        return;
    }
    state.metrics.messages_out.fetch_add(1, Ordering::Relaxed);

    fanout(&state, &sender, server_id, joined).await;

    if let Some(phase) = phase_event {
        fanout(&state, &sender, server_id, phase).await;
    }

    let mut heartbeat = interval(Duration::from_secs(10));
    let mut window_start = Instant::now();
    let mut action_count = 0u32;

    loop {
        select! {
            _ = heartbeat.tick() => {
                let message = {
                    let guard = room.read().await;
                    guard.heartbeat(server_id)
                };
                if write.send(Message::Text(message)).await.is_err() {
                    break;
                }
                state.metrics.messages_out.fetch_add(1, Ordering::Relaxed);
            }
            incoming = read.next() => {
                match incoming {
                    Some(Ok(Message::Text(text))) => {
                        state.metrics.messages_in.fetch_add(1, Ordering::Relaxed);

                        let payload = match serde_json::from_str::<ClientEnvelope>(&text) {
                            Ok(payload) => payload,
                            Err(_) => {
                                let _ = write.send(Message::Text(RoomRuntime::encode_error("invalid_payload", &format!("payload must match ClientEnvelope for {actor_label}")))).await;
                                state.metrics.messages_out.fetch_add(1, Ordering::Relaxed);
                                continue;
                            }
                        };
                        match payload.event {
                            ClientEvent::Join { .. } => {
                                let (_welcome, snapshot, _joined, phase_event) = {
                                    let mut guard = room.write().await;
                                    guard.join(server_id, player_id)
                                };
                                if write.send(Message::Text(snapshot)).await.is_err() {
                                    break;
                                }
                                state.metrics.messages_out.fetch_add(1, Ordering::Relaxed);
                                if let Some(phase) = phase_event {
                                    fanout(&state, &sender, server_id, phase).await;
                                }
                            }
                            ClientEvent::Action { kind, payload } => {
                                if window_start.elapsed() > StdDuration::from_secs(1) {
                                    window_start = Instant::now();
                                    action_count = 0;
                                }
                                action_count = action_count.saturating_add(1);
                                if action_count > 20 {
                                    state.metrics.dropped_actions.fetch_add(1, Ordering::Relaxed);
                                    let _ = write.send(Message::Text(RoomRuntime::encode_error("rate_limited", "too many actions per second"))).await;
                                    state.metrics.messages_out.fetch_add(1, Ordering::Relaxed);
                                    continue;
                                }

                                let event = {
                                    let mut guard = room.write().await;
                                    guard.action(server_id, player_id, kind, payload)
                                };

                                match event {
                                    Ok(message) => {
                                        fanout(&state, &sender, server_id, message).await;
                                    }
                                    Err(error) => {
                                        let _ = write.send(Message::Text(error)).await;
                                        state.metrics.messages_out.fetch_add(1, Ordering::Relaxed);
                                    }
                                }
                            }
                            ClientEvent::Ping { client_time } => {
                                let pong = {
                                    let guard = room.read().await;
                                    guard.pong(server_id, client_time)
                                };
                                if write.send(Message::Text(pong)).await.is_err() {
                                    break;
                                }
                                state.metrics.messages_out.fetch_add(1, Ordering::Relaxed);
                            }
                            ClientEvent::Leave => {
                                break;
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        break;
                    }
                    Some(Ok(_)) => {}
                    Some(Err(_)) => break,
                }
            }
            outbound = receiver.recv() => {
                match outbound {
                    Ok(message) => {
                        if write.send(Message::Text(message)).await.is_err() {
                            break;
                        }
                        state.metrics.messages_out.fetch_add(1, Ordering::Relaxed);
                    }
                    Err(_) => break,
                }
            }
        }
    }

    let left = {
        let mut guard = room.write().await;
        guard.leave(server_id, player_id)
    };

    if let Some((event, phase_event)) = left {
        fanout(&state, &sender, server_id, event).await;
        if let Some(phase) = phase_event {
            fanout(&state, &sender, server_id, phase).await;
        }
    }

    state.metrics.active_connections.fetch_sub(1, Ordering::Relaxed);
}

async fn fanout(
    state: &AppState,
    sender: &tokio::sync::broadcast::Sender<String>,
    server_id: Uuid,
    message: String,
) {
    let sent = sender.send(message.clone()).is_ok();
    if sent {
        state.metrics.messages_out.fetch_add(1, Ordering::Relaxed);
    }
    distributed::publish(state, server_id, &message).await;
}
