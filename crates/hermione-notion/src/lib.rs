mod client;

pub use client::*;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Notion API request error: {status}, {message}")]
    ApiError { status: u16, message: String },

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[error(transparent)]
    Unknown(#[from] eyre::Report),
}
