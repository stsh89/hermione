use crate::providers::powershell::{self, PowerShellProcess};
use hermione_nexus::{
    services::{ClipboardProvider, CopyCommandToClipboard},
    Result,
};

pub struct Clipboard<'a> {
    pub process: &'a PowerShellProcess,
}

impl ClipboardProvider for Clipboard<'_> {}

impl CopyCommandToClipboard for Clipboard<'_> {
    fn copy_command_to_clipboard(&self, text: &str) -> Result<()> {
        powershell::copy_to_clipboard(self.process, text)?;

        Ok(())
    }
}
