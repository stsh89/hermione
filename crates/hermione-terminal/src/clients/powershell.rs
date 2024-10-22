use crate::Result;
use hermione_powershell::{Client, PowerShellParameters};

pub struct PowerShellClient {
    inner: Client,
}

pub struct PowerShellOpenWindowsTerminalClientParameters<'a> {
    pub command: Option<&'a str>,
    pub no_exit: bool,
    pub working_directory: &'a str,
}

impl PowerShellClient {
    pub fn copy_to_clipboard(&self, text: &str) -> Result<()> {
        self.inner.copy_to_clipboard(text)
    }

    pub fn new() -> Result<Self> {
        Ok(Self {
            inner: Client::new()?,
        })
    }

    pub fn open_windows_terminal(
        &self,
        parameters: PowerShellOpenWindowsTerminalClientParameters,
    ) -> Result<()> {
        let PowerShellOpenWindowsTerminalClientParameters {
            command,
            no_exit,
            working_directory,
        } = parameters;

        self.inner.open_windows_terminal(Some(PowerShellParameters {
            command,
            no_exit,
            working_directory: Some(working_directory),
        }))
    }
}
