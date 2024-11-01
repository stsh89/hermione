use crate::{
    definitions::{CommandId, WorkspaceId},
    services::{CopyCommandToClipboard, FindCommand},
    Error, Result,
};

pub struct CopyCommandToClipboardOperation<'a, FCP, CP> {
    pub find_command_provider: &'a FCP,
    pub clipboard_provider: &'a CP,
}

impl<'a, FCP, CP> CopyCommandToClipboardOperation<'a, FCP, CP>
where
    FCP: FindCommand,
    CP: CopyCommandToClipboard,
{
    pub fn execute(&self, workspace_id: &WorkspaceId, id: &CommandId) -> Result<()> {
        tracing::info!(operation = "Copy command to clipboard");

        let command = self.find_command_provider.find_command(id)?;

        let Some(command) = command else {
            return Err(Error::NotFound(format!("Command with ID: {}", **id)));
        };

        if command.workspace_id() != workspace_id {
            return Err(Error::InvalidArgument(
                "Command doesn't belong to workspace".to_string(),
            ));
        }

        self.clipboard_provider
            .copy_command_to_clipboard(command.program())?;

        Ok(())
    }
}
