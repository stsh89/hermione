#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    AlreadyExists(String),

    #[error("{0}")]
    DataLoss(String),

    #[error("{0}")]
    FailedPrecondition(String),

    #[error("{0}")]
    Internal(String),

    #[error("{0} not found")]
    NotFound(String),

    #[error(transparent)]
    Unknown(#[from] eyre::Report),
}
