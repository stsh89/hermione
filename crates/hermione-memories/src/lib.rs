mod date_time;
mod id;

pub mod entities;
pub mod operations;

pub use date_time::*;
pub use id::*;

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
