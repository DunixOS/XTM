#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TokenRecord {
    pub gamertag: String,
    pub xuid: String,
    pub status: String,
    pub expiration: String,
    pub token: String,
}
