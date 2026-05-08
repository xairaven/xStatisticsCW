#[derive(Debug, thiserror::Error)]
pub enum InputError {
    #[error("Failed to parse input as integer: {0}.")]
    Parse(#[from] std::num::ParseIntError),

    #[error("Input cannot be empty.")]
    Empty,

    #[error("A must be lower than B.")]
    Order,
}
