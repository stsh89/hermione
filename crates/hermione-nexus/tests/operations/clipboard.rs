use std::sync::{PoisonError, RwLock};

use hermione_nexus::{
    services::{ClipboardProvider, CopyCommandToClipboard},
    Error,
};

#[derive(thiserror::Error, Debug)]
pub enum MockClipboardError {
    #[error("Lock access: {0}")]
    LockAccess(String),
}

pub struct MockClipboardProvider {
    pub content: RwLock<Option<String>>,
}

impl ClipboardProvider for MockClipboardProvider {}

impl MockClipboardProvider {
    pub fn new() -> Self {
        Self {
            content: RwLock::new(None),
        }
    }

    pub fn set_content(&self, text: &str) -> Result<(), MockClipboardError> {
        *self.content.write()? = Some(text.to_string());

        Ok(())
    }

    pub fn content(&self) -> Result<Option<String>, MockClipboardError> {
        let text = self.content.read()?;

        Ok(text.clone())
    }
}

impl CopyCommandToClipboard for MockClipboardProvider {
    fn copy_command_to_clipboard(&self, text: &str) -> Result<(), Error> {
        self.set_content(text)?;

        Ok(())
    }
}

impl<T> From<PoisonError<T>> for MockClipboardError {
    fn from(err: PoisonError<T>) -> Self {
        Self::LockAccess(err.to_string())
    }
}

impl From<MockClipboardError> for Error {
    fn from(_err: MockClipboardError) -> Self {
        Error::Internal("Can't access mock clipboard".to_string())
    }
}
