use crate::{
    definitions::CommandId,
    services::{DeleteCommand, FindCommand},
    Error, Result,
};

pub struct DeleteCommandOperation<'a, FCP, GCP> {
    pub find_command_provider: &'a FCP,
    pub delete_command_provider: &'a GCP,
}

impl<'a, FCP, GCP> DeleteCommandOperation<'a, FCP, GCP>
where
    FCP: FindCommand,
    GCP: DeleteCommand,
{
    pub fn execute(&self, id: &CommandId) -> Result<()> {
        tracing::info!(operation = "Delete command");

        self.find_command_provider
            .find_command(id)?
            .ok_or(Error::NotFound(format!("Command {}", id.braced())))?;

        self.delete_command_provider.delete_command(id)?;

        Ok(())
    }
}
