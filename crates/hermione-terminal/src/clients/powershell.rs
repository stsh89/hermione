use crate::Result;
use anyhow::anyhow;
use hermione_powershell::Client as InnerClient;

pub use hermione_powershell::WindowsTerminalParameters;

pub struct Client {
    inner: InnerClient,
}

impl Client {
    pub fn copy_to_clipboard(self, text: &str) -> Result<()> {
        self.inner
            .copy_to_clipboard(text)
            .map_err(|err| anyhow!(err))
    }

    pub fn new() -> Result<Self> {
        Ok(Self {
            inner: InnerClient::new().map_err(|err| anyhow!(err))?,
        })
    }

    pub fn start_windows_terminal(self, parameters: WindowsTerminalParameters) -> Result<()> {
        self.inner
            .start_windows_terminal(parameters)
            .map_err(|err| anyhow!(err))
    }
}
