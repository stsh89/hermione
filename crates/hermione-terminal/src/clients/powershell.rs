use crate::Result;
use anyhow::anyhow;
use std::sync::{RwLock, RwLockWriteGuard};

pub struct PowerShell {
    inner: RwLock<hermione_powershell::Client>,
}

pub struct WindowsTerminalParameters<'a> {
    pub directory: &'a str,
    pub command: Option<&'a str>,
    pub no_exit: bool,
}

impl PowerShell {
    fn client(&self) -> Result<RwLockWriteGuard<'_, hermione_powershell::Client>> {
        self.inner.write().map_err(|err| anyhow!("{}", err))
    }

    pub fn copy_to_clipboard(&self, text: &str) -> Result<()> {
        self.client()?
            .copy_to_clipboard(text)
            .map_err(|err| anyhow!(err))
    }

    pub fn new() -> Result<Self> {
        Ok(Self {
            inner: RwLock::new(hermione_powershell::Client::new()?),
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
