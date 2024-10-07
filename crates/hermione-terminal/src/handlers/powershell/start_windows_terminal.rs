use crate::{brokers, parameters::powershell::start_windows_terminal::Parameters, Result};

pub struct Handler<'a> {
    pub powershell: &'a brokers::powershell::Broker,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<()> {
        let Parameters { working_directory } = parameters;

        self.powershell
            .start_windows_terminal(brokers::powershell::WindowsTerminalParameters {
                command: None,
                directory: working_directory.as_deref(),
                no_exit: true,
            })?;

        Ok(())
    }
}
