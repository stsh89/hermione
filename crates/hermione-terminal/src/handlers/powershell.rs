use crate::{
    features::{
        CopyToClipboardOperation, ExecuteCommandOperation, ExecuteCommandParameters,
        OpenWindowsTerminalOperation,
    },
    Coordinator, CopyToClipboardParameters, OpenWindowsTerminalParameters,
    PowerShellExecuteCommandParameters, Result,
};
use hermione_powershell::PowerShellProvider;

pub struct PowerShellHandler<'a> {
    pub coordinator: &'a Coordinator,
    pub powershell: &'a PowerShellProvider,
}

impl<'a> PowerShellHandler<'a> {
    pub fn copy_to_clipboard(self, parameters: CopyToClipboardParameters) -> Result<()> {
        let CopyToClipboardParameters {
            workspace_id,
            command_id,
        } = parameters;

        let command = self
            .coordinator
            .commands()
            .get(&workspace_id, &command_id)?;

        CopyToClipboardOperation {
            clipboard_provider: self.powershell,
        }
        .execute(&command.program)
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

        ExecuteCommandOperation {
            runner: self.powershell,
            tracker: self.coordinator.commands(),
        }
        .execute(ExecuteCommandParameters {
            command: &command,
            no_exit: powershell_no_exit,
            working_directory: workspace.location.as_str(),
        })?;

        Ok(())
    }

    pub fn open_windows_terminal(self, parameters: OpenWindowsTerminalParameters) -> Result<()> {
        let OpenWindowsTerminalParameters { working_directory } = parameters;

        OpenWindowsTerminalOperation {
            windows_terminal_provider: self.powershell,
        }
        .execute(&working_directory)
    }
}
