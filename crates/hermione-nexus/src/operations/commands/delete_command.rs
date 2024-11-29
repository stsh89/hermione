use super::GetCommandOperation;
use crate::{
    definitions::{Command, CommandId},
    services::{DeleteCommand, FindCommand, StorageService},
    Result,
};

pub struct DeleteCommandOperation<'a, FCP, GCP>
where
    FCP: StorageService,
    GCP: StorageService,
{
    pub find_provider: &'a FCP,
    pub delete_provider: &'a GCP,
}

impl<'a, FCP, GCP> DeleteCommandOperation<'a, FCP, GCP>
where
    FCP: FindCommand,
    GCP: DeleteCommand,
{
    pub fn execute(&self, id: CommandId) -> Result<()> {
        tracing::info!(operation = "Delete command");

        self.get_command(id)?;
        self.delete_provider.delete_command(id)?;

        Ok(())
    }

    fn get_command(&self, id: CommandId) -> Result<Command> {
        GetCommandOperation {
            provider: self.find_provider,
        }
        .execute(id)
    }
}
