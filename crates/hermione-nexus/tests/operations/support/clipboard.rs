use eyre::eyre;
use hermione_nexus::{
    services::{ClipboardService, CopyCommandToClipboard},
    Error, Result,
};
use std::sync::RwLock;

#[derive(Default)]
pub struct MockClipboard {
    pub content: RwLock<Option<String>>,
}

impl MockClipboard {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn set_content(&self, text: &str) -> Result<()> {
        let mut content = self.content.write().map_err(|_err| {
            Error::Clipboard(eyre!(
                "Shared memory blocked for writing, can proceeed with copying"
            ))
        })?;

        *content = Some(text.to_string());

        Ok(())
    }
}

impl ClipboardService for MockClipboard {}

impl CopyCommandToClipboard for MockClipboard {
    fn copy_command_to_clipboard(&self, text: &str) -> Result<()> {
        self.set_content(text)?;

        Ok(())
    }
}
