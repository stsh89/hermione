mod client;
mod provider;

pub mod json;

pub use client::*;
pub use provider::*;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Notion API request error: {status}, {message}")]
    ApiError { status: u16, message: String },

    #[error("{0}")]
    Internal(String),

    #[error("{0}")]
    InvalidArgument(String),

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

impl Error {
    fn internal(message: &str) -> Self {
        Self::Internal(message.into())
    }

    fn invalid_argument(message: &str) -> Self {
        Self::InvalidArgument(message.into())
    }

    fn unexpected<E>(error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Unexpected(eyre::Error::new(error))
    }
}
