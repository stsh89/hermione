use crate::{
    clients, coordinator::Coordinator, parameters::powershell::execute_command::Parameters, Result,
};

pub struct Handler<'a> {
    pub coordinator: &'a Coordinator,
    pub powershell: &'a clients::powershell::PowerShell,
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

        self.powershell.open_windows_terminal(
            clients::powershell::OpenWindowsTerminalParameters {
                working_directory: workspace.location.as_str(),
                no_exit: powershell_no_exit,
                command: Some(command.program.as_str()),
            },
        )?;

        self.coordinator
            .workspaces()
            .commands()
            .track_execution_time(command)?;

        Ok(())
    }
}
