use crate::{clients, parameters::powershell::start_windows_terminal::Parameters, Result};

pub struct Handler<'a> {
    pub powershell: &'a clients::powershell::PowerShell,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<()> {
        let Parameters { working_directory } = parameters;

        self.powershell
            .start_windows_terminal(clients::powershell::WindowsTerminalParameters {
                command: None,
                directory: &working_directory,
                no_exit: true,
            })?;

        Ok(())
    }
}
