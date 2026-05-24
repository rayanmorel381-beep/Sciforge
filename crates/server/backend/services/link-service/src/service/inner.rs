use chrono::{Duration, Utc};
use link_protocol::{IssueLinkCodeRequest, IssueLinkCodeResponse, LinkStatus, LinkStatusResponse};
use uuid::Uuid;

use crate::record::LinkRecord;

pub fn create_record(payload: IssueLinkCodeRequest) -> (String, LinkRecord, IssueLinkCodeResponse) {
    let code = new_link_code();
    let record = LinkRecord {
        binding_id: Uuid::new_v4(),
        code: code.clone(),
        apk_session_id: payload.apk_session_id,
        browser_session_id: None,
        account_id: payload.account_id,
        expires_at: Utc::now() + Duration::minutes(5),
    };
    let response = IssueLinkCodeResponse {
        code: record.code.clone(),
        expires_at: record.expires_at,
        binding_id: record.binding_id,
    };
    (code, record, response)
}

pub fn to_status_response(record: &LinkRecord) -> LinkStatusResponse {
    let status = if record.expires_at < Utc::now() {
        LinkStatus::Expired
    } else if record.browser_session_id.is_some() {
        LinkStatus::Claimed
    } else {
        LinkStatus::Pending
    };

    LinkStatusResponse {
        binding_id: record.binding_id,
        code: record.code.clone(),
        status,
        apk_session_id: record.apk_session_id.clone(),
        browser_session_id: record.browser_session_id.clone(),
        account_id: record.account_id,
        expires_at: record.expires_at,
    }
}

fn new_link_code() -> String {
    let raw = Uuid::new_v4().simple().to_string().to_uppercase();
    raw.chars().take(6).collect()
}