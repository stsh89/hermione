use crate::{
    clients::powershell, parameters::powershell::start_windows_terminal::Parameters, Result,
};

pub struct Handler {}

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
