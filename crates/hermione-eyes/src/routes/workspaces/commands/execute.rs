use crate::{app::ExecuteCommandParameters, clients::memories, types::Result};
use hermione_wand::clients::powershell::{Client, StartWindowsTerminalParameters};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: ExecuteCommandParameters) -> Result<()> {
        let ExecuteCommandParameters {
            workspace_id,
            command_id,
        } = parameters;

        let command = self.memories.get_command(&workspace_id, &command_id)?;
        let workspace = self.memories.get_workspace(&workspace_id)?;
        let powershell = Client::new()?;

        powershell.start_windows_terminal(StartWindowsTerminalParameters {
            directory: Some(&workspace.location),
            no_exit: true,
            command: Some(&command.program),
        })?;

        self.memories.track_command_execution_time(command)?;

        Ok(())
    }
}
