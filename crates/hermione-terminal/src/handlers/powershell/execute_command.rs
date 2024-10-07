use crate::{integrations, parameters::powershell::execute_command::Parameters, Result};

pub struct Handler<'a> {
    pub workspaces: &'a integrations::core::workspaces::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<()> {
        let Parameters {
            workspace_id,
            command_id,
            powershell_no_exit,
        } = parameters;

        let command = self.workspaces.commands().get(&workspace_id, &command_id)?;
        let workspace = self.workspaces.get(&workspace_id)?;
        let powershell = integrations::powershell::Client::new()?;

        powershell.start_windows_terminal(integrations::powershell::WindowsTerminalParameters {
            directory: workspace.location.as_deref(),
            no_exit: powershell_no_exit,
            command: Some(&command.program),
        })?;

        self.workspaces.commands().track_execution_time(command)?;

        Ok(())
    }
}
