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

#[derive(Default)]
pub struct MockClipboard {
    content: RwLock<Option<String>>,
}

impl MockClipboard {
    pub fn content(&self) -> Result<Option<String>, MockClipboardError> {
        let text = self.content.read()?;

        Ok(text.clone())
    }

    pub fn set_content(&self, text: &str) -> Result<(), MockClipboardError> {
        *self.content.write()? = Some(text.to_string());

        Ok(())
    }
}

impl<T> From<PoisonError<T>> for MockClipboardError {
    fn from(err: PoisonError<T>) -> Self {
        Self::LockAccess(err.to_string())
    }
}

impl From<MockClipboardError> for Error {
    fn from(err: MockClipboardError) -> Self {
        Error::Clipboard(eyre::Error::new(err))
    }
}

impl ClipboardProvider for MockClipboard {}

impl CopyCommandToClipboard for MockClipboard {
    fn copy_command_to_clipboard(&self, text: &str) -> Result<(), Error> {
        self.set_content(text)?;

        Ok(())
    }
}
