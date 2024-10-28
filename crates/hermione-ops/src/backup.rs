use crate::{commands::Command, workspaces::Workspace, Result};
use std::future::Future;
use tracing::instrument;
use uuid::Uuid;

pub trait BckImportCommand {
    fn bck_import_command(&self, command: Command) -> impl Future<Output = Result<Command>>;
}

pub trait BckImportWorkspace {
    fn bck_import_workspace(&self, workspace: Workspace)
        -> impl Future<Output = Result<Workspace>>;
}

pub trait BckIterateCommands {
    fn bck_iterate_commands(&self) -> impl Future<Output = Result<Option<Vec<Command>>>>;
}

pub trait BckIterateWorkspaces {
    fn bck_iterate_workspaces(&self) -> impl Future<Output = Result<Option<Vec<Workspace>>>>;
}

pub trait BckListCommands {
    fn bck_list_commands(&self, ids: Vec<Uuid>) -> impl Future<Output = Result<Vec<Command>>>;
}

pub trait BckListWorkspaces {
    fn bck_list_workspaces(&self, ids: Vec<Uuid>) -> impl Future<Output = Result<Vec<Workspace>>>;
}

pub trait BckUpdateCommand {
    fn bck_update_command(&self, command: Command) -> impl Future<Output = Result<Command>>;
}

pub trait BckUpdateWorkspace {
    fn bck_update_workspace(&self, workspace: Workspace)
        -> impl Future<Output = Result<Workspace>>;
}

pub struct BackupCommandsOperation<'a, ICP, IMCP, LCP, UCP> {
    pub iterate_commands_provider: &'a ICP,
    pub import_command_provider: &'a IMCP,
    pub list_commands_provider: &'a LCP,
    pub update_command_provider: &'a UCP,
}

pub struct BackupWorkspacesOperation<'a, IWP, IMWP, LWP, UWP> {
    pub iterate_workspaces_provider: &'a IWP,
    pub import_workspace_provider: &'a IMWP,
    pub list_workspaces_provider: &'a LWP,
    pub update_workspace_provider: &'a UWP,
}

impl<'a, ICP, IMCP, LCP, UCP> BackupCommandsOperation<'a, ICP, IMCP, LCP, UCP>
where
    ICP: BckIterateCommands,
    IMCP: BckImportCommand,
    LCP: BckListCommands,
    UCP: BckUpdateCommand,
{
    #[instrument(skip(self))]
    pub async fn execute(&self) -> Result<()> {
        tracing::info!("Backup commands started");

        while let Some(commands) = self
            .iterate_commands_provider
            .bck_iterate_commands()
            .await?
        {
            let backuped_commands = self.list_backuped_commands(&commands).await?;

            for command in commands {
                let backuped_command = backuped_commands.iter().find(|r| r.id() == command.id());

                let Some(backuped_command) = backuped_command else {
                    self.import_command_provider
                        .bck_import_command(command)
                        .await?;

                    continue;
                };

                if &command != backuped_command {
                    self.update_command_provider
                        .bck_update_command(command)
                        .await?;
                }
            }
        }

        Ok(())
    }

    async fn list_backuped_commands(&self, commands: &[Command]) -> Result<Vec<Command>> {
        self.list_commands_provider
            .bck_list_commands(commands.iter().filter_map(|c| c.id()).collect())
            .await
    }
}

impl<'a, IWP, IMWP, LWP, UWP> BackupWorkspacesOperation<'a, IWP, IMWP, LWP, UWP>
where
    IWP: BckIterateWorkspaces,
    IMWP: BckImportWorkspace,
    LWP: BckListWorkspaces,
    UWP: BckUpdateWorkspace,
{
    #[instrument(skip(self))]
    pub async fn execute(&self) -> Result<()> {
        tracing::info!("Backup workspaces started");

        while let Some(workspaces) = self
            .iterate_workspaces_provider
            .bck_iterate_workspaces()
            .await?
        {
            let backuped_workspaces = self.list_backuped_workspaces(&workspaces).await?;

            for workspace in workspaces {
                let backuped_workspace = backuped_workspaces
                    .iter()
                    .find(|r| r.id() == workspace.id());

                let Some(backuped_workspace) = backuped_workspace else {
                    self.import_workspace_provider
                        .bck_import_workspace(workspace)
                        .await?;

                    continue;
                };

                if &workspace != backuped_workspace {
                    self.update_workspace_provider
                        .bck_update_workspace(workspace)
                        .await?;
                }
            }
        }

        Ok(())
    }

    async fn list_backuped_workspaces(&self, workspaces: &[Workspace]) -> Result<Vec<Workspace>> {
        self.list_workspaces_provider
            .bck_list_workspaces(workspaces.iter().filter_map(|c| c.id()).collect())
            .await
    }
}

pub struct BackupOperation<'a, ICP, IWP, IMCP, IMWP, LCP, LWP, UCP, UWP> {
    pub iterate_commands_provider: &'a ICP,
    pub iterate_workspaces_provider: &'a IWP,
    pub import_command_provider: &'a IMCP,
    pub import_workspace_provider: &'a IMWP,
    pub list_commands_provider: &'a LCP,
    pub list_workspaces_provider: &'a LWP,
    pub update_command_provider: &'a UCP,
    pub update_workspace_provider: &'a UWP,
}

impl<'a, ICP, IWP, IMCP, IMWP, LCP, LWP, UCP, UWP>
    BackupOperation<'a, ICP, IWP, IMCP, IMWP, LCP, LWP, UCP, UWP>
where
    ICP: BckIterateCommands,
    IWP: BckIterateWorkspaces,
    IMCP: BckImportCommand,
    IMWP: BckImportWorkspace,
    LCP: BckListCommands,
    LWP: BckListWorkspaces,
    UCP: BckUpdateCommand,
    UWP: BckUpdateWorkspace,
{
    #[instrument(skip(self))]
    pub async fn execute(&self) -> Result<()> {
        BackupWorkspacesOperation {
            iterate_workspaces_provider: self.iterate_workspaces_provider,
            import_workspace_provider: self.import_workspace_provider,
            list_workspaces_provider: self.list_workspaces_provider,
            update_workspace_provider: self.update_workspace_provider,
        }
        .execute()
        .await?;

        BackupCommandsOperation {
            iterate_commands_provider: self.iterate_commands_provider,
            import_command_provider: self.import_command_provider,
            list_commands_provider: self.list_commands_provider,
            update_command_provider: self.update_command_provider,
        }
        .execute()
        .await?;

        Ok(())
    }
}
