use crate::{
    definitions::CommandId,
    operations::GetCommandOperation,
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

        GetCommandOperation {
            provider: self.find_command_provider,
        }
        .execute(id)?;

        self.delete_command_provider.delete_command(id)?;

        Ok(())
    }
}
