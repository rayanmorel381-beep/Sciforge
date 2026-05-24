use serde::Deserialize;
use uuid::Uuid;

use server_control::Visibility;

#[derive(Debug, Deserialize)]
pub struct ListServersQuery {
    pub owner_account_id: Option<Uuid>,
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Deserialize)]
pub struct AccessQuery {
    pub account_id: Uuid,
    pub invite_code: Option<String>,
}