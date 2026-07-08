use thiserror::Error;

#[derive(Debug, Error)]
pub enum XboxApiError {
    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("authentication expired for token: {0}")]
    ExpiredToken(String),
    #[error("request failed with status {status}: {body}")]
    RequestStatus { status: u16, body: String },
    #[error("missing data: {0}")]
    MissingData(String),
}

impl From<anyhow::Error> for XboxApiError {
    fn from(value: anyhow::Error) -> Self {
        Self::Serialization(value.to_string())
    }
}
