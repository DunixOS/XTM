use thiserror::Error;

#[derive(Debug, Error)]
pub enum XboxError {
    #[error("http request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("failed to parse response: {0}")]
    Parse(String),
    #[error("missing required data: {0}")]
    MissingData(String),
    #[error("not configured: {0}")]
    NotConfigured(String),
}

impl From<anyhow::Error> for XboxError {
    fn from(value: anyhow::Error) -> Self {
        Self::Parse(value.to_string())
    }
}
