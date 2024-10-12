use crate::Result;
use anyhow::anyhow;
use hermione_powershell::Client;
use std::sync::{RwLock, RwLockWriteGuard};

pub struct Broker {
    client: RwLock<Client>,
}

pub struct WindowsTerminalParameters<'a> {
    pub directory: &'a str,
    pub command: Option<&'a str>,
    pub no_exit: bool,
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
        let WindowsTerminalParameters {
            directory,
            command,
            no_exit,
        } = parameters;

        let directory = if directory.is_empty() {
            None
        } else {
            Some(directory)
        };

        self.client()?
            .start_windows_terminal(hermione_powershell::WindowsTerminalParameters {
                directory,
                command,
                no_exit,
            })
            .map_err(|err| anyhow!(err))
    }
}
