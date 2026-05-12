use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Wolfram API returned an HTTP error status: {0}")]
    Http(u16),

    #[error("Wolfram API logical error: {0}")]
    Wolfram(String),
}
