use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Parse float error: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Job queue error: {0}")]
    JobQueue(String),

    #[error("Triage error: {0}")]
    Triage(String),

    #[error("Span processing error: {0}")]
    SpanProcessing(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("External service error: {0}")]
    ExternalService(String),

    #[error("System command error: {0}")]
    SystemCommand(String),
}
