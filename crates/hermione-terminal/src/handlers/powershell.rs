use crate::{
    Coordinator, PowerShellClient, PowerShellCopyToClipboardParameters,
    PowerShellExecuteCommandParameters, PowerShellOpenWindowsTerminalClientParameters,
    PowerShellOpenWindowsTerminalParameters, Result,
};

pub struct PowerShellHandler<'a> {
    pub coordinator: &'a Coordinator,
    pub powershell: &'a PowerShellClient,
}

impl<'a> PowerShellHandler<'a> {
    pub fn copy_to_clipboard(self, parameters: PowerShellCopyToClipboardParameters) -> Result<()> {
        let PowerShellCopyToClipboardParameters {
            workspace_id,
            command_id,
        } = parameters;

        let command = self
            .coordinator
            .commands()
            .get(&workspace_id, &command_id)?;

        self.powershell.copy_to_clipboard(&command.program)
    }

    pub fn execute_command(self, parameters: PowerShellExecuteCommandParameters) -> Result<()> {
        let PowerShellExecuteCommandParameters {
            workspace_id,
            command_id,
            powershell_no_exit,
        } = parameters;

        let command = self
            .coordinator
            .commands()
            .get(&workspace_id, &command_id)?;

        let workspace = self.coordinator.workspaces().get(&workspace_id)?;

        self.powershell
            .open_windows_terminal(PowerShellOpenWindowsTerminalClientParameters {
                working_directory: workspace.location.as_str(),
                no_exit: powershell_no_exit,
                command: Some(command.program.as_str()),
            })?;

        self.coordinator.commands().track_execution_time(command)?;

        Ok(())
    }

    pub fn open_windows_terminal(
        self,
        parameters: PowerShellOpenWindowsTerminalParameters,
    ) -> Result<()> {
        let PowerShellOpenWindowsTerminalParameters { working_directory } = parameters;

        self.powershell
            .open_windows_terminal(PowerShellOpenWindowsTerminalClientParameters {
                command: None,
                working_directory: &working_directory,
                no_exit: true,
            })?;

        Ok(())
    }
}
