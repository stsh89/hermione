use crate::{
    coordinator::{Coordinator, ExecuteCommandInput, ExecuteProgramInput},
    CopyCommandToClipboardParams, ExecuteCommandParams, ExecuteProgramParams,
    OpenWindowsTerminalParams, Result,
};

pub struct PowerShellHandler<'a> {
    pub coordinator: &'a Coordinator,
}

impl<'a> PowerShellHandler<'a> {
    pub fn copy_to_clipboard(self, parameters: CopyCommandToClipboardParams) -> Result<()> {
        let CopyCommandToClipboardParams { command_id } = parameters;

        self.coordinator.copy_command_to_clipboard(command_id)?;

        Ok(())
    }

    pub fn execute_command(self, params: ExecuteCommandParams) -> Result<()> {
        let ExecuteCommandParams {
            command_id,
            powershell_no_exit: no_exit,
        } = params;

        self.coordinator.execute_command(ExecuteCommandInput {
            command_id,
            no_exit,
        })?;

        Ok(())
    }

    pub fn execute_program(self, params: ExecuteProgramParams) -> Result<()> {
        let ExecuteProgramParams {
            workspace_id,
            program,
        } = params;

        self.coordinator.execute_program(ExecuteProgramInput {
            workspace_id,
            program,
        })?;

        Ok(())
    }

    pub fn open_windows_terminal(self, params: OpenWindowsTerminalParams) -> Result<()> {
        let OpenWindowsTerminalParams { workspace_id } = params;

        self.coordinator.open_windows_terminal(workspace_id)?;

        Ok(())
    }
}
