use std::sync::Arc;
use std::time::{Duration, Instant};

use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tokio::time::timeout;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Debug, Deserialize)]
struct AuthResponse {
    account_id: String,
    token: String,
}

#[derive(Debug, Deserialize)]
struct ServerResponse {
    id: String,
    realtime_endpoint: String,
}

#[derive(Default)]
struct Stats {
    connected: usize,
    failed_connections: usize,
    action_acknowledged: usize,
    action_errors: usize,
    latencies_ms: Vec<u128>,
}

#[tokio::main]
async fn main() {
    let concurrency = read_env_usize("LOAD_RUNNER_CONNECTIONS", 50);
    let api_base = std::env::var("API_GATEWAY_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
    let client = reqwest::Client::new();

    let owner = register_user(&client, &api_base, "owner").await;
    let server = create_server(&client, &api_base, &owner.account_id).await;

    let stats = Arc::new(Mutex::new(Stats::default()));
    let mut tasks = JoinSet::new();

    for index in 0..concurrency {
        let client = client.clone();
        let api_base = api_base.clone();
        let endpoint = server.realtime_endpoint.clone();
        let server_id = server.id.clone();
        let stats = stats.clone();
        tasks.spawn(async move {
            run_client(client, api_base, endpoint, server_id, index, stats).await;
        });
    }

    while tasks.join_next().await.is_some() {}

    let stats = stats.lock().await;
    let average_latency_ms = if stats.latencies_ms.is_empty() {
        0.0
    } else {
        stats.latencies_ms.iter().sum::<u128>() as f64 / stats.latencies_ms.len() as f64
    };
    let p95_latency_ms = percentile(&stats.latencies_ms, 95.0);

    println!(
        "load_runner connections={} connected={} failed_connections={} action_acknowledged={} action_errors={} avg_latency_ms={:.2} p95_latency_ms={}"
            ,
        concurrency,
        stats.connected,
        stats.failed_connections,
        stats.action_acknowledged,
        stats.action_errors,
        average_latency_ms,
        p95_latency_ms,
    );
}

async fn run_client(
    client: reqwest::Client,
    api_base: String,
    endpoint: String,
    server_id: String,
    index: usize,
    stats: Arc<Mutex<Stats>>,
) {
    let user = register_user(&client, &api_base, &format!("player_{index}")).await;
    let ws_url = format!("{}?token={}", endpoint, user.token);

    let connect_started = Instant::now();
    let Ok((mut socket, _)) = connect_async(ws_url).await else {
        let mut guard = stats.lock().await;
        guard.failed_connections += 1;
        return;
    };

    {
        let mut guard = stats.lock().await;
        guard.connected += 1;
        guard.latencies_ms.push(connect_started.elapsed().as_millis());
    }

    if !wait_until_running(&mut socket).await {
        let mut guard = stats.lock().await;
        guard.action_errors += 1;
        let _ = socket.close(None).await;
        return;
    }

    let send_started = Instant::now();
    if socket
        .send(Message::Text(
            json!({
                "event": {
                    "type": "action",
                    "kind": "move",
                    "payload": {"x": index as i64, "y": 1}
                }
            })
            .to_string(),
        ))
        .await
        .is_err()
    {
        let mut guard = stats.lock().await;
        guard.action_errors += 1;
        return;
    }

    let mut acknowledged = false;
    for _ in 0..12 {
        let Ok(Some(Ok(Message::Text(text)))) = timeout(Duration::from_secs(2), socket.next()).await else {
            continue;
        };
        if is_action_broadcast(&text, &server_id) {
            let mut guard = stats.lock().await;
            guard.action_acknowledged += 1;
            guard.latencies_ms.push(send_started.elapsed().as_millis());
            acknowledged = true;
            break;
        }
        if is_error(&text) {
            let mut guard = stats.lock().await;
            guard.action_errors += 1;
            acknowledged = true;
            break;
        }
    }

    if !acknowledged {
        let mut guard = stats.lock().await;
        guard.action_errors += 1;
    }

    let _ = socket.close(None).await;
}

async fn register_user(client: &reqwest::Client, api_base: &str, label: &str) -> AuthResponse {
    client
        .post(format!("{api_base}/api/auth/register"))
        .json(&json!({
            "username": format!("{}_{}", label, uuid::Uuid::new_v4().simple()),
            "device_id": format!("device_{}", uuid::Uuid::new_v4().simple()),
        }))
        .send()
        .await
        .unwrap()
        .json::<AuthResponse>()
        .await
        .unwrap()
}

async fn create_server(client: &reqwest::Client, api_base: &str, owner_account_id: &str) -> ServerResponse {
    client
        .post(format!("{api_base}/api/servers"))
        .json(&json!({
            "owner_account_id": owner_account_id,
            "link_code": "LOAD42",
            "name": "Load Arena",
            "region": "eu-west",
            "mode": "mass",
            "visibility": "public",
            "max_players": 128
        }))
        .send()
        .await
        .unwrap()
        .json::<ServerResponse>()
        .await
        .unwrap()
}

fn read_env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

fn is_action_broadcast(payload: &str, server_id: &str) -> bool {
    let Ok(value) = serde_json::from_str::<Value>(payload) else {
        return false;
    };
    value["event"]["type"] == "action_broadcast" && value["event"]["server_id"] == server_id
}

async fn wait_until_running(
    socket: &mut tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
) -> bool {
    for _ in 0..16 {
        let Ok(Some(Ok(Message::Text(text)))) = timeout(Duration::from_secs(2), socket.next()).await else {
            continue;
        };
        let Ok(value) = serde_json::from_str::<Value>(&text) else {
            continue;
        };
        if value["event"]["type"] == "phase_changed" && value["event"]["phase"] == "running" {
            return true;
        }
        if value["event"]["type"] == "presence_snapshot"
            && value["event"]["players"].as_array().map(|players| players.len()).unwrap_or(0) >= 2
        {
            return true;
        }
    }
    false
}

fn is_error(payload: &str) -> bool {
    let Ok(value) = serde_json::from_str::<Value>(payload) else {
        return false;
    };
    value["event"]["type"] == "error"
}

fn percentile(values: &[u128], percentile: f64) -> u128 {
    if values.is_empty() {
        return 0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let position = ((percentile / 100.0) * (sorted.len().saturating_sub(1)) as f64).round() as usize;
    sorted[position]
}