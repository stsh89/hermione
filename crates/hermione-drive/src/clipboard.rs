use hermione_internals::powershell::{self, PowerShellProcess};
use hermione_nexus::{
    services::{ClipboardService, CopyCommandToClipboard},
    Result,
};

pub struct Clipboard<'a> {
    process: &'a PowerShellProcess,
}

impl<'a> Clipboard<'a> {
    pub fn new(process: &'a PowerShellProcess) -> Self {
        Clipboard { process }
    }
}

impl ClipboardService for Clipboard<'_> {}

impl CopyCommandToClipboard for Clipboard<'_> {
    fn copy_command_to_clipboard(&self, text: &str) -> Result<()> {
        powershell::copy_to_clipboard(self.process, text)
    }
}
