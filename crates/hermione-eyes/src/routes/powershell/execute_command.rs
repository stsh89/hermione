use crate::{
    clients::{
        memories::Client,
        powershell::{self, StartWindowsTerminalParameters},
    },
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a Client,
}

pub struct Parameters {
    pub command_id: String,
    pub workspace_id: String,
    pub powershell_no_exit: bool,
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
        let powershell = powershell::Client::new()?;

        powershell.start_windows_terminal(StartWindowsTerminalParameters {
            directory: workspace.location.as_deref(),
            no_exit: powershell_no_exit,
            command: Some(&command.program),
        })?;

        self.memories.track_command_execution_time(command)?;

        Ok(())
    }
}
