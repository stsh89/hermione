#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Internal: {0}")]
    Internal(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error(transparent)]
    Storage(#[from] eyre::Error),
}
