use crate::{
    definitions::{Command, CommandId, Workspace, WorkspaceId},
    operations::{GetCommandOperation, GetWorkspaceOperation},
    services::{
        FindCommand, FindWorkspace, InvokeCommand, InvokeCommandParameters, StorageService,
        SystemService, TrackCommandExecuteTime, TrackWorkspaceAccessTime,
    },
    Result,
};

pub struct ExecuteCommandOperation<'a, FCP, FWP, SP, TCP, TWP>
where
    FCP: StorageService,
    FWP: StorageService,
    SP: SystemService,
    TCP: StorageService,
    TWP: StorageService,
{
    pub find_command_provider: &'a FCP,
    pub find_workspace_provider: &'a FWP,
    pub system_provider: &'a SP,
    pub track_command_provider: &'a TCP,
    pub track_workspace_provider: &'a TWP,
}

impl<'a, FCP, FWP, SP, TCP, TWP> ExecuteCommandOperation<'a, FCP, FWP, SP, TCP, TWP>
where
    FCP: FindCommand,
    FWP: FindWorkspace,
    SP: InvokeCommand,
    TCP: TrackCommandExecuteTime,
    TWP: TrackWorkspaceAccessTime,
{
    pub fn execute(&self, id: &CommandId) -> Result<()> {
        tracing::info!(operation = "Execute command");

        let command = self.get_command(id)?;
        let workspace = self.get_workspace(command.workspace_id())?;

        self.system_provider
            .invoke_command(InvokeCommandParameters {
                command: command.program(),
                location: workspace.location(),
            })?;

        self.track_command_provider.track_command_execute_time(id)?;
        self.track_workspace_provider
            .track_workspace_access_time(command.workspace_id())?;

        Ok(())
    }

    fn get_command(&self, id: &CommandId) -> Result<Command> {
        GetCommandOperation {
            provider: self.find_command_provider,
        }
        .execute(id)
    }

    fn get_workspace(&self, id: &WorkspaceId) -> Result<Workspace> {
        GetWorkspaceOperation {
            provider: self.find_workspace_provider,
        }
        .execute(id)
    }
}
