use crate::Result;
use anyhow::anyhow;
use hermione_powershell::Client;

pub use hermione_powershell::WindowsTerminalParameters;

pub struct Broker;

impl Broker {
    pub fn copy_to_clipboard(&self, text: &str) -> Result<()> {
        Client::new()?
            .copy_to_clipboard(text)
            .map_err(|err| anyhow!(err))
    }

    pub fn new() -> Self {
        Self
    }

    pub fn start_windows_terminal(&self, parameters: WindowsTerminalParameters) -> Result<()> {
        Client::new()?
            .start_windows_terminal(parameters)
            .map_err(|err| anyhow!(err))
    }
}
