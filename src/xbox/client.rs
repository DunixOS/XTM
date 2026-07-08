use std::time::Duration;

use reqwest::Client;

use crate::xbox::error::XboxError;

#[derive(Debug, Clone)]
pub struct XboxClient {
    inner: Client,
    base_url: String,
}

impl Default for XboxClient {
    fn default() -> Self {
        Self::new("https://xboxapi.xboxlive.com")
    }
}

impl XboxClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        let inner = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("reqwest client should be built");
        Self {
            inner,
            base_url: base_url.into(),
        }
    }

    pub async fn get_profile(&self, xuid: &str) -> Result<serde_json::Value, XboxError> {
        let url = format!("{}/users/xuid({xuid})", self.base_url);
        let response = self.inner.get(&url).send().await?;
        let response = response.error_for_status().map_err(|err| XboxError::Request(err.into()))?;
        response.json::<serde_json::Value>().await.map_err(|err| XboxError::Parse(err.to_string()))
    }
}
