use crate::Result;
use anyhow::anyhow;
use hermione_powershell::Client;
use std::sync::{RwLock, RwLockWriteGuard};

pub use hermione_powershell::WindowsTerminalParameters;

pub struct Broker {
    client: RwLock<Client>,
}

impl Broker {
    fn client(&self) -> Result<RwLockWriteGuard<'_, Client>> {
        self.client.write().map_err(|err| anyhow!("{}", err))
    }

    pub fn copy_to_clipboard(&self, text: &str) -> Result<()> {
        self.client()?
            .copy_to_clipboard(text)
            .map_err(|err| anyhow!(err))
    }

    pub fn new() -> Result<Self> {
        Ok(Self {
            client: RwLock::new(Client::new()?),
        })
    }

    pub fn start_windows_terminal(&self, parameters: WindowsTerminalParameters) -> Result<()> {
        self.client()?
            .start_windows_terminal(parameters)
            .map_err(|err| anyhow!(err))
    }
}
