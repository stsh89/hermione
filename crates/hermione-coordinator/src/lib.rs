mod core;
pub mod workspaces;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    FailedPrecondition(String),

    #[error("{0}")]
    Internal(String),

    #[error(transparent)]
    Unknown(#[from] eyre::Report),
}

impl From<hermione_core::Error> for Error {
    fn from(value: hermione_core::Error) -> Self {
        match value {
            hermione_core::Error::FailedPrecondition(msg) => Self::FailedPrecondition(msg),
            hermione_core::Error::Internal(msg) => Self::Internal(msg),
            hermione_core::Error::Unknown(err) => Self::Unknown(err),
        }
    }
}
