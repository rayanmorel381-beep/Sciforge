use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EventsQuery {
    pub token: String,
    pub invite_code: Option<String>,
}
