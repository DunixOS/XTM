use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum FollowEvent {
    Started { gamertag: String },
    Completed { gamertag: String, succeeded: bool },
    Expired { gamertag: String, token: String },
    Finished { progress: FollowProgress },
}

#[derive(Debug, Clone)]
pub enum MessageEvent {
    Completed { recipient: String, succeeded: bool, detail: String },
}

#[derive(Debug, Clone, Serialize)]
pub struct XstsAuthorizeRequest {
    pub properties: XstsAuthorizeProperties,
    pub relying_party: String,
    pub token_type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct XstsAuthorizeProperties {
    pub sandbox_id: String,
    pub user_tokens: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct XstsAuthorizeResponse {
    pub token: String,
    pub display_claims: DisplayClaims,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DisplayClaims {
    pub xui: Vec<XuiEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct XuiEntry {
    pub uhs: String,
}

#[derive(Debug, Clone)]
pub struct FollowProgress {
    pub total_tokens: usize,
    pub successful_follows: usize,
    pub failed_follows: usize,
    pub expired_tokens: usize,
    pub progress: f32,
    pub current_target: String,
    pub log: Vec<String>,
}
