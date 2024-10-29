use crate::{
    coordinator::{Coordinator, ExecuteCommandWithinWorkspaceInput, OpenWindowsTerminalInput},
    CopyToClipboardParams, ExecuteCommandParams, OpenWindowsTerminalParams, Result,
};

pub struct PowerShellHandler<'a> {
    pub coordinator: &'a Coordinator<'a>,
}

impl<'a> PowerShellHandler<'a> {
    pub fn copy_to_clipboard(self, parameters: CopyToClipboardParams) -> Result<()> {
        let CopyToClipboardParams {
            workspace_id,
            command_id,
        } = parameters;

        self.coordinator
            .copy_program_to_clipboard(&workspace_id, &command_id)?;

        Ok(())
    }

    pub fn execute_command(self, params: ExecuteCommandParams) -> Result<()> {
        let ExecuteCommandParams {
            workspace_id,
            command_id,
            powershell_no_exit: no_exit,
        } = params;

        self.coordinator
            .execute_command(ExecuteCommandWithinWorkspaceInput {
                id: &command_id,
                workspace_id: &workspace_id,
                no_exit,
            })?;

        Ok(())
    }

    pub fn open_windows_terminal(self, params: OpenWindowsTerminalParams) -> Result<()> {
        let OpenWindowsTerminalParams { working_directory } = params;

        self.coordinator
            .open_windows_terminal(OpenWindowsTerminalInput {
                working_directory: &working_directory,
            })?;

        Ok(())
    }
}
