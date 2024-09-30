use crate::{router::powershell::StartWindowsTerminalParameters, types::Result};
use hermione_wand::clients::powershell;

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
