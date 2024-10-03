mod base;

pub mod entities;
pub mod operations;

pub use base::*;

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
