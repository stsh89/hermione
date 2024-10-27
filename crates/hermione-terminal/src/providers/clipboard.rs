use hermione_ops::{extensions::CopyCommandToClipboard, Result};
use hermione_powershell::PowerShellClient;

pub struct ClipboardProvider<'a> {
    pub client: &'a PowerShellClient,
}

impl CopyCommandToClipboard for ClipboardProvider<'_> {
    fn copy_command_to_clipboard(&self, text: &str) -> Result<()> {
        self.client
            .copy_to_clipboard(text)
            .map_err(eyre::Error::new)?;

        Ok(())
    }
}
