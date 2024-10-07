use crate::{
    brokers, coordinator::Coordinator, parameters::powershell::execute_command::Parameters, Result,
};

pub struct Handler<'a> {
    pub coordinator: &'a Coordinator,
    pub powershell: &'a brokers::powershell::Broker,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<()> {
        let Parameters {
            workspace_id,
            command_id,
            powershell_no_exit,
        } = parameters;

        let command = self
            .coordinator
            .workspaces()
            .commands()
            .get(&workspace_id, &command_id)?;
        let workspace = self.coordinator.workspaces().get(&workspace_id)?;

        self.powershell
            .start_windows_terminal(brokers::powershell::WindowsTerminalParameters {
                directory: workspace.location.as_deref(),
                no_exit: powershell_no_exit,
                command: Some(&command.program),
            })?;

        self.coordinator
            .workspaces()
            .commands()
            .track_execution_time(command)?;

        Ok(())
    }
}
