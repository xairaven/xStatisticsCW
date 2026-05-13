use crate::config::ConfigError;
use crate::logs::LogsError;
use crate::ui::GraphicsBackendError;
use crate::ui::errors::InputError;
use backend::BackendError;
use opener::OpenError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrontendError {
    #[error("Configuration. {0}")]
    Config(#[from] ConfigError),

    #[error("Graphics Backend. {0}")]
    GraphicsBackend(#[from] GraphicsBackendError),

    #[error("Logger. {0}")]
    Logs(#[from] LogsError),

    #[error("Input. {0}")]
    Input(#[from] InputError),

    #[error("Backend. {0}")]
    Backend(#[from] BackendError),

    #[error("IO. {0}")]
    IO(std::io::Error),

    #[error("Opener. {0}")]
    Opener(OpenError),
}
