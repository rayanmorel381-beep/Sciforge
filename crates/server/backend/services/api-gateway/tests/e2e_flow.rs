use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug, Deserialize)]
struct AuthResponse {
    account_id: String,
    token: String,
    refresh_token: String,
}

#[derive(Debug, Deserialize)]
struct IssueCodeResponse {
    code: String,
}

#[derive(Debug, Deserialize)]
struct ServerResponse {
    id: String,
    realtime_endpoint: String,
}

#[tokio::test]
#[ignore]
async fn e2e_register_link_create_server_and_realtime() {
    let api_base = std::env::var("API_GATEWAY_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
    let realtime_base = std::env::var("REALTIME_GATEWAY_URL").unwrap_or_else(|_| "http://127.0.0.1:8085".to_string());

    let client = reqwest::Client::new();

    let register_a = client
        .post(format!("{api_base}/api/auth/register"))
        .json(&json!({
            "username": format!("owner_{}", uuid::Uuid::new_v4().simple()),
            "device_id": "apk-a"
        }))
        .send()
        .await
        .unwrap();
    assert!(register_a.status().is_success());
    let auth_a: AuthResponse = register_a.json().await.unwrap();

    let register_b = client
        .post(format!("{api_base}/api/auth/register"))
        .json(&json!({
            "username": format!("guest_{}", uuid::Uuid::new_v4().simple()),
            "device_id": "apk-b"
        }))
        .send()
        .await
        .unwrap();
    assert!(register_b.status().is_success());
    let auth_b: AuthResponse = register_b.json().await.unwrap();

    let refresh = client
        .post(format!("{api_base}/api/auth/refresh"))
        .json(&json!({"refresh_token": auth_a.refresh_token}))
        .send()
        .await
        .unwrap();
    assert!(refresh.status().is_success());

    let issue_code = client
        .post(format!("{api_base}/api/link/codes"))
        .json(&json!({
            "apk_session_id": format!("apk-{}", uuid::Uuid::new_v4().simple()),
            "account_id": auth_a.account_id
        }))
        .send()
        .await
        .unwrap();
    assert!(issue_code.status().is_success());
    let issued: IssueCodeResponse = issue_code.json().await.unwrap();

    let claim = client
        .post(format!("{api_base}/api/link/codes/claim"))
        .json(&json!({
            "code": issued.code,
            "browser_session_id": format!("browser-{}", uuid::Uuid::new_v4().simple()),
            "account_id": auth_a.account_id
        }))
        .send()
        .await
        .unwrap();
    assert!(claim.status().is_success());

    let create_server = client
        .post(format!("{api_base}/api/servers"))
        .json(&json!({
            "owner_account_id": auth_a.account_id,
            "link_code": "ABC123",
            "name": "E2E Arena",
            "region": "eu-west",
            "mode": "duo",
            "visibility": "public",
            "max_players": 8
        }))
        .send()
        .await
        .unwrap();
    assert!(create_server.status().is_success());
    let server: ServerResponse = create_server.json().await.unwrap();

    let ws_url_a = format!("{}?token={}", server.realtime_endpoint, auth_a.token);
    let ws_url_b = format!("{}?token={}", server.realtime_endpoint, auth_b.token);

    let (mut ws_a, _) = tokio_tungstenite::connect_async(ws_url_a).await.unwrap();
    let (mut ws_b, _) = tokio_tungstenite::connect_async(ws_url_b).await.unwrap();

    let mut saw_phase_running = false;
    for _ in 0..8 {
        let Some(Ok(Message::Text(text))) = timeout(Duration::from_secs(2), ws_a.next()).await.unwrap() else {
            continue;
        };
        if is_phase_running(&text) {
            saw_phase_running = true;
            break;
        }
    }
    assert!(saw_phase_running);

    ws_a
        .send(Message::Text(
            json!({
                "event": {
                    "type": "action",
                    "kind": "move",
                    "payload": {"x": 1, "y": 2}
                }
            })
            .to_string(),
        ))
        .await
        .unwrap();

    let mut saw_action = false;
    for _ in 0..12 {
        let Some(Ok(Message::Text(text))) = timeout(Duration::from_secs(2), ws_b.next()).await.unwrap() else {
            continue;
        };
        if is_action_broadcast(&text, &server.id) {
            saw_action = true;
            break;
        }
    }
    assert!(saw_action);

    let metrics = client
        .get(format!("{realtime_base}/metrics"))
        .send()
        .await
        .unwrap();
    assert!(metrics.status().is_success());
    let body: Value = metrics.json().await.unwrap();
    assert!(body["messages_in"].as_u64().unwrap_or(0) >= 1);

    let _ = ws_a.send(Message::Close(None)).await;
    let _ = ws_b.send(Message::Close(None)).await;
}

fn is_phase_running(payload: &str) -> bool {
    let Ok(value) = serde_json::from_str::<Value>(payload) else {
        return false;
    };
    value["event"]["type"] == "phase_changed" && value["event"]["phase"] == "running"
}

fn is_action_broadcast(payload: &str, server_id: &str) -> bool {
    let Ok(value) = serde_json::from_str::<Value>(payload) else {
        return false;
    };
    value["event"]["type"] == "action_broadcast" && value["event"]["server_id"] == server_id
}
