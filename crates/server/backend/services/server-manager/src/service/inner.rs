use chrono::Utc;
use server_control::{CreateServerRequest, ServerState, ServerSummary, Visibility};
use uuid::Uuid;

pub fn build_server(payload: CreateServerRequest) -> ServerSummary {
    let now = Utc::now();
    let server_id = Uuid::new_v4();
    ServerSummary {
        id: server_id,
        owner_account_id: payload.owner_account_id,
        link_code: payload.link_code,
        join_code: new_join_code(),
        access_code: build_access_code(payload.visibility.clone()),
        realtime_endpoint: build_realtime_endpoint(server_id),
        name: payload.name.trim().to_string(),
        region: payload.region.trim().to_string(),
        mode: payload.mode.trim().to_string(),
        visibility: payload.visibility,
        max_players: payload.max_players.clamp(2, 128),
        state: ServerState::Online,
        created_at: now,
        updated_at: now,
    }
}

fn build_access_code(visibility: Visibility) -> Option<String> {
    match visibility {
        Visibility::Private | Visibility::InviteOnly => {
            Some(Uuid::new_v4().simple().to_string().chars().take(6).collect::<String>().to_uppercase())
        }
        _ => None,
    }
}

fn new_join_code() -> String {
    Uuid::new_v4().simple().to_string().chars().take(8).collect::<String>().to_uppercase()
}

fn build_realtime_endpoint(server_id: Uuid) -> String {
    let base = std::env::var("REALTIME_BASE_URL").unwrap_or_else(|_| "ws://127.0.0.1:8085".to_string());
    format!("{base}/realtime/servers/{server_id}/events")
}