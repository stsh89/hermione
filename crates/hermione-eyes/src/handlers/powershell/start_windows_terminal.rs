use crate::{clients, parameters::powershell::start_windows_terminal::Parameters, Result};

pub struct Handler {}

impl Handler {
    pub fn handle(self, parameters: Parameters) -> Result<()> {
        let Parameters { working_directory } = parameters;

        let powershell = clients::powershell::Client::new()?;

        powershell.start_windows_terminal(clients::powershell::WindowsTerminalParameters {
            command: None,
            directory: working_directory.as_deref(),
            no_exit: true,
        })?;

        Ok(())
    }
}
