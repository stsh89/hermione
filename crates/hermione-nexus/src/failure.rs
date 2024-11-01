#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Internal: {0}")]
    Internal(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error(transparent)]
    Storage(#[from] eyre::Error),
}
