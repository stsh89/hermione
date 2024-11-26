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
}

impl ClipboardService for MockClipboard {}

impl CopyCommandToClipboard for MockClipboard {
    fn copy_command_to_clipboard(&self, text: &str) -> Result<()> {
        set_clipboard_content(self, text).map_err(|err| {
            Error::unexpected(err.wrap_err("Could not copy command to clipboard"))
        })?;

        Ok(())
    }
}

fn set_clipboard_content(clipboard: &MockClipboard, text: &str) -> eyre::Result<()> {
    let mut content = clipboard
        .content
        .write()
        .map_err(|_err| eyre!("Clipboard memory write access failure"))?;

    *content = Some(text.to_string());

    Ok(())
}
