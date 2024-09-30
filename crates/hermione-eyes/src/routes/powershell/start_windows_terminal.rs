use crate::{app::router::powershell::StartWindowsTerminalParameters, clients::powershell, Result};

pub struct Handler {}

impl Handler {
    pub fn handle(self, parameters: StartWindowsTerminalParameters) -> Result<()> {
        let StartWindowsTerminalParameters { working_directory } = parameters;

        let powershell = powershell::Client::new()?;

        powershell.start_windows_terminal(powershell::StartWindowsTerminalParameters {
            command: None,
            directory: working_directory.as_deref(),
            no_exit: true,
        })?;

        Ok(())
    }
}
