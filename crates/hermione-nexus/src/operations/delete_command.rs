use crate::{
    definitions::CommandId,
    services::{DeleteCommand, FindCommand, StorageProvider},
    Error, Result,
};

pub struct DeleteCommandOperation<'a, FCP, GCP>
where
    FCP: StorageProvider,
    GCP: StorageProvider,
{
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

        if self.find_command_provider.find_command(id)?.is_none() {
            return Err(Error::NotFound(format!("Command {}", id.braced())));
        }

        self.delete_command_provider.delete_command(id)
    }
}
