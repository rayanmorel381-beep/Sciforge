#[derive(Clone)]
pub struct AppState {
    pub client: reqwest::Client,
    pub server_manager_url: String,
}

impl AppState {
    pub fn new(server_manager_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            server_manager_url,
        }
    }
}