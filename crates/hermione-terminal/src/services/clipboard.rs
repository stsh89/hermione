use crate::providers::PowerShellClient;
use hermione_nexus::{
    services::{ClipboardProvider, CopyCommandToClipboard},
    Result,
};

pub struct Clipboard<'a> {
    pub client: &'a PowerShellClient,
}

impl ClipboardProvider for Clipboard<'_> {}

impl CopyCommandToClipboard for Clipboard<'_> {
    fn copy_command_to_clipboard(&self, text: &str) -> Result<()> {
        self.client.copy_to_clipboard(text)?;

        Ok(())
    }
}
