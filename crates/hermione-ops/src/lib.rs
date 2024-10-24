use std::io;

pub mod backup;
pub mod commands;
pub mod extensions;
pub mod notion;
pub mod workspaces;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Data loss: {0}")]
    DataLoss(String),

    #[error("{0}")]
    FailedPrecondition(String),

    #[error("{0}")]
    Internal(String),

    #[error("{0}")]
    IO(#[source] io::Error),

    #[error("Not found {0}")]
    NotFound(String),

    #[error(transparent)]
    Unknown(#[from] eyre::Report),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}
