use crate::{
    definitions::{Command, CommandId},
    operations::GetCommandOperation,
    services::{FindCommand, SetClipboardContent, StorageService, SystemService},
    Result,
};

pub struct CopyCommandToClipboardOperation<'a, SP, CP>
where
    CP: SystemService,
    SP: StorageService,
{
    pub clipboard_provider: &'a CP,
    pub storage_provider: &'a SP,
}

impl<'a, FC, CCP> CopyCommandToClipboardOperation<'a, FC, CCP>
where
    FC: FindCommand,
    CCP: SetClipboardContent,
{
    pub fn execute(&self, id: CommandId) -> Result<()> {
        tracing::info!(operation = "Copy command to clipboard");

        let command = self.get_command(id)?;

        self.clipboard_provider
            .set_clipboard_content(command.program())
    }

    fn get_command(&self, id: CommandId) -> Result<Command> {
        GetCommandOperation {
            provider: self.storage_provider,
        }
        .execute(id)
    }
}
