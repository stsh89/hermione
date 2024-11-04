use crate::{
    definitions::{Command, CommandId},
    operations::GetCommandOperation,
    services::{ClipboardProvider, CopyCommandToClipboard, FindCommand, StorageProvider},
    Result,
};

pub struct CopyCommandToClipboardOperation<'a, SP, CP>
where
    CP: ClipboardProvider,
    SP: StorageProvider,
{
    pub clipboard_provider: &'a CP,
    pub storage_provider: &'a SP,
}

impl<'a, FC, CCP> CopyCommandToClipboardOperation<'a, FC, CCP>
where
    FC: FindCommand,
    CCP: CopyCommandToClipboard,
{
    pub fn execute(&self, id: &CommandId) -> Result<()> {
        tracing::info!(operation = "Copy command to clipboard");

        let command = self.get_command(id)?;

        self.clipboard_provider
            .copy_command_to_clipboard(command.program())
    }

    fn get_command(&self, id: &CommandId) -> Result<Command> {
        GetCommandOperation {
            provider: self.storage_provider,
        }
        .execute(id)
    }
}
