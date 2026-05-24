#[derive(Clone)]
pub struct AppState {
    pub client: reqwest::Client,
    pub identity_url: String,
    pub link_service_url: String,
    pub server_manager_url: String,
    pub lobby_url: String,
}

impl AppState {
    pub fn from_env() -> Self {
        Self {
            client: reqwest::Client::new(),
            identity_url: std::env::var("IDENTITY_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".to_string()),
            link_service_url: std::env::var("LINK_SERVICE_URL").unwrap_or_else(|_| "http://127.0.0.1:8082".to_string()),
            server_manager_url: std::env::var("SERVER_MANAGER_URL").unwrap_or_else(|_| "http://127.0.0.1:8083".to_string()),
            lobby_url: std::env::var("LOBBY_URL").unwrap_or_else(|_| "http://127.0.0.1:8084".to_string()),
        }
    }
}