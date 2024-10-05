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

impl From<hermione_memories::Error> for Error {
    fn from(value: hermione_memories::Error) -> Self {
        match value {
            hermione_memories::Error::FailedPrecondition(msg) => Self::FailedPrecondition(msg),
            hermione_memories::Error::Internal(msg) => Self::Internal(msg),
            hermione_memories::Error::Unknown(err) => Self::Unknown(err),
        }
    }
}
