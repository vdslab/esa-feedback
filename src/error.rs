use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP client error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Webhook validation error: {0}")]
    WebhookValidation(String),

    #[error("Vertex AI error: {0}")]
    VertexAi(String),

    #[error("Esa API error: {0}")]
    EsaApi(String),

    #[error("Pattern matching error: {0}")]
    #[allow(dead_code)]
    PatternMatching(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;
