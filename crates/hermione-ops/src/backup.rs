use crate::{
    commands::{Command, FindCommand, ImportCommand, ListAllCommandsInBatches, UpdateCommand},
    workspaces::{
        FindWorkspace, ImportWorkspace, ListAllWorkspacesInBatches, UpdateWorkspace, Workspace,
    },
    Error, Result,
};

pub struct BackupOperation<'a, C, RC, RW, W> {
    pub commands: &'a C,
    pub remote_commands: &'a RC,
    pub remote_workspaces: &'a RW,
    pub workspaces: &'a W,
}

impl<'a, C, RC, RW, W> BackupOperation<'a, C, RC, RW, W>
where
    C: ListAllCommandsInBatches,
    RC: FindCommand + ImportCommand + UpdateCommand,
    RW: FindWorkspace + ImportWorkspace + UpdateWorkspace,
    W: ListAllWorkspacesInBatches,
{
    pub async fn execute(&self) -> Result<()> {
        self.backup_workspaces().await?;
        self.backup_commands().await?;

        Ok(())
    }

    async fn backup_commands(&self) -> Result<()> {
        self.commands
            .list_all_commands_in_batches(|batch| self.backup_commands_batch(batch))
            .await?;

        Ok(())
    }

    fn backup_commands_batch(&self, batch: Vec<Command>) -> Result<()> {
        for command in batch {
            let command_id = command.id().ok_or(Error::DataLoss("Command ID".into()))?;

            let remote_command = self.remote_commands.find_command(command_id)?;

            let Some(remote_command) = remote_command else {
                self.remote_commands.import_command(command)?;
                continue;
            };

            if command != remote_command {
                self.remote_commands.update_command(command)?;
            }
        }

        Ok(())
    }

    async fn backup_workspaces(&self) -> Result<()> {
        self.workspaces
            .list_all_workspaces_in_batches(|batch| self.backup_workspaces_batch(batch))
            .await?;

        Ok(())
    }

    fn backup_workspaces_batch(&self, batch: Vec<Workspace>) -> Result<()> {
        for workspace in batch {
            let id = workspace
                .id()
                .ok_or(Error::DataLoss("Workspace ID".into()))?;

            let remote_workspace = self.remote_workspaces.find_workspace(id)?;

            let Some(remote_workspace) = remote_workspace else {
                self.remote_workspaces.import_workspace(workspace)?;
                continue;
            };

            if workspace != remote_workspace {
                self.remote_workspaces.update_workspace(workspace)?;
            }
        }

        Ok(())
    }
}
