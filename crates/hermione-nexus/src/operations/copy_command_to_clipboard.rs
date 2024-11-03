use crate::{
    definitions::CommandId,
    operations::GetCommandOperation,
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
    pub fn execute(&self, id: &CommandId) -> Result<()> {
        tracing::info!(operation = "Copy command to clipboard");

        let command = GetCommandOperation {
            provider: self.find_command_provider,
        }
        .execute(id)?;

        self.clipboard_provider
            .copy_command_to_clipboard(command.program())?;

        Ok(())
    }
}
