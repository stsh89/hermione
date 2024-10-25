use std::future::Future;
use uuid::Uuid;

use crate::{
    commands::{Command, FindCommand, ListCommandsParameters, UpdateCommand},
    workspaces::{FindWorkspace, ListAllWorkspacesInBatches, UpdateWorkspace, Workspace},
    Error, Result,
};

pub trait CommandsIterator {
    fn iterate_over_commands<M, MR>(&self, map_fn: M) -> impl Future<Output = Result<()>>
    where
        M: Fn(Vec<Command>) -> MR,
        MR: Future<Output = Result<()>>;
}

pub trait ImportCommand {
    fn import_command(&self, command: Command) -> Result<Command>;
}

pub trait ImportWorkspace {
    fn import_workspace(&self, workspace: Workspace) -> Result<Workspace>;
}

pub trait ListCommands {
    fn list_commands(&self, parameters: ListCommandsParameters) -> Result<Vec<Command>>;
}

pub trait ListAllCommandsInBatches {
    fn list_all_commands_in_batches(
        &self,
        batch_fn: impl Fn(Vec<Command>) -> Result<()>,
    ) -> impl Future<Output = Result<()>>;
}

pub struct BackupCommandOperation<'a, CP, RCP> {
    pub commands_provider: &'a CP,
    pub remote_commands_provider: &'a RCP,
}

pub struct BackupOperation<'a, C, RC, RW, W> {
    pub commands: &'a C,
    pub remote_commands: &'a RC,
    pub remote_workspaces: &'a RW,
    pub workspaces: &'a W,
}

pub struct ImportCommandOperation<'a, S> {
    pub importer: &'a S,
}

pub struct ImportWorkspaceOperation<'a, S> {
    pub importer: &'a S,
}

impl<'a, CP, RCP> BackupCommandOperation<'a, CP, RCP>
where
    CP: CommandsIterator,
    RCP: ImportCommand + ListCommands + UpdateCommand,
{
    async fn execute(&self) -> Result<()> {
        self.commands_provider
            .iterate_over_commands(|commands| async {
                let remote_commands = self.list_remote_commands(&commands)?;

                for command in commands {
                    let remote_command = remote_commands.iter().find(|c| c.id() == command.id());

                    let Some(remote_command) = remote_command else {
                        self.remote_commands_provider.import_command(command)?;
                        continue;
                    };

                    if &command != remote_command {
                        self.remote_commands_provider.update_command(command)?;
                    }
                }

                Ok(())
            })
            .await?;

        Ok(())
    }

    fn list_remote_commands(&self, commands: &[Command]) -> Result<Vec<Command>> {
        self.remote_commands_provider
            .list_commands(ListCommandsParameters {
                page_number: 0,
                page_size: commands.len() as u32,
                ids: commands
                    .iter()
                    .map(|c| c.try_id())
                    .collect::<Result<Vec<Uuid>>>()?,
            })
    }
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

impl<'a, S> ImportCommandOperation<'a, S>
where
    S: ImportCommand,
{
    pub fn execute(&self, command: Command) -> Result<Command> {
        self.importer.import_command(command)
    }
}

impl<'a, S> ImportWorkspaceOperation<'a, S>
where
    S: ImportWorkspace,
{
    pub fn execute(&self, workspace: Workspace) -> Result<Workspace> {
        self.importer.import_workspace(workspace)
    }
}

pub struct ListCommandsOperation<'a, L>
where
    L: ListCommands,
{
    pub lister: &'a L,
}

impl<'a, L> ListCommandsOperation<'a, L>
where
    L: ListCommands,
{
    pub fn execute(&self, parameters: ListCommandsParameters) -> Result<Vec<Command>> {
        self.lister.list_commands(parameters)
    }
}
