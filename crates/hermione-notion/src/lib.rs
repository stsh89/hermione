mod client;

pub use client::*;

pub type Result<T> = std::result::Result<T, Error>;
pub type Json = serde_json::Value;

pub trait NotionRichTextProperty {
    fn rich_text(&self, property_name: &str) -> &str;
}

pub trait NotionTitlePropery {
    fn title(&self) -> &str;
}

pub trait NotionPageId {
    fn id(&self) -> &str;
}

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

impl NotionRichTextProperty for Json {
    fn rich_text(&self, property_name: &str) -> &str {
        self["properties"][property_name]["rich_text"][0]["plain_text"]
            .as_str()
            .unwrap_or_default()
    }
}

impl NotionTitlePropery for Json {
    fn title(&self) -> &str {
        self["properties"]["Name"]["title"][0]["plain_text"]
            .as_str()
            .unwrap_or_default()
    }
}

impl NotionPageId for Json {
    fn id(&self) -> &str {
        self["id"].as_str().unwrap_or_default()
    }
}
