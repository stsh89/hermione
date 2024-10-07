use crate::{brokers, clients, parameters::powershell::execute_command::Parameters, Result};

pub struct Handler<'a> {
    pub memories: &'a clients::memories::Client,
    pub powershell: &'a brokers::powershell::Broker,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<()> {
        let Parameters {
            workspace_id,
            command_id,
            powershell_no_exit,
        } = parameters;

        let command = self.memories.get_command(&workspace_id, &command_id)?;
        let workspace = self.memories.get_workspace(&workspace_id)?;

        self.powershell
            .start_windows_terminal(brokers::powershell::WindowsTerminalParameters {
                directory: workspace.location.as_deref(),
                no_exit: powershell_no_exit,
                command: Some(&command.program),
            })?;

        self.memories.track_command_execution_time(command)?;

        Ok(())
    }
}
