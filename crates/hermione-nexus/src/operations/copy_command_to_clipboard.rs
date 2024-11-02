use crate::{
    definitions::CommandId,
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

        let command = self.find_command_provider.find_command(id)?;

        let Some(command) = command else {
            return Err(Error::NotFound(format!("Command with ID: {}", **id)));
        };

        self.clipboard_provider
            .copy_command_to_clipboard(command.program())?;

        Ok(())
    }
}
