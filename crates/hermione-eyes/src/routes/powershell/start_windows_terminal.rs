use crate::{clients::powershell, Result};

pub struct Handler {}

pub struct Parameters {
    pub working_directory: Option<String>,
}

impl Handler {
    pub fn handle(self, parameters: Parameters) -> Result<()> {
        let Parameters { working_directory } = parameters;

        let powershell = powershell::Client::new()?;

        powershell.start_windows_terminal(powershell::StartWindowsTerminalParameters {
            command: None,
            directory: working_directory.as_deref(),
            no_exit: true,
        })?;

        Ok(())
    }
}
