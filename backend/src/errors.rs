use api::ApiError;
use thiserror::Error;
use tokio::sync::AcquireError;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("API Request failed: {0}")]
    Api(#[from] ApiError),

    #[error("Missing required pod: {0}")]
    MissingPod(String),

    #[error("Failed to parse math result. {0}")]
    Parse(String),

    #[error("Failed to acquire semaphore. {0}")]
    Semaphore(AcquireError),

    #[error("Tokio Runtime. {0}")]
    Tokio(std::io::Error),
}
