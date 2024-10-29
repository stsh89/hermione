use crate::{StorageProvider, StorageProviderResult};
use chrono::Utc;
use hermione_ops::{
    commands::{Command, CommandId, GetCommandFromWorkspace},
    extensions::{TrackCommandExecutionTime, TrackWorkspaceAccessTime},
    workspaces::{GetWorkspace, Workspace, WorkspaceId},
    Result,
};
use rusqlite::params;

impl<'a> StorageProvider<'a> {
    fn track_command_execution_time(
        &self,
        workspace_id: &WorkspaceId,
        id: &CommandId,
    ) -> StorageProviderResult<()> {
        let last_execute_time = Utc::now().timestamp_nanos_opt();

        self.connection
            .prepare(
                "UPDATE commands
                SET last_execute_time = ?1
                WHERE id = ?2 AND workspace_id = ?3",
            )?
            .execute(params![
                last_execute_time,
                id.as_bytes(),
                workspace_id.as_bytes()
            ])?;

        Ok(())
    }

    fn track_workspace_access_time(&self, id: &WorkspaceId) -> StorageProviderResult<()> {
        let last_access_time = Utc::now().timestamp_nanos_opt();

        let mut statement = self.connection.prepare(
            "UPDATE workspaces
            SET last_access_time = ?1
            WHERE id = ?2",
        )?;

        statement.execute(params![last_access_time, id.as_bytes()])?;

        Ok(())
    }
}

impl TrackCommandExecutionTime for StorageProvider<'_> {
    fn track_command_execution_time(&self, command: Command) -> Result<Command> {
        self.track_command_execution_time(command.workspace_id(), command.id())?;

        GetCommandFromWorkspace::get_command_from_workspace(
            self,
            command.workspace_id(),
            command.id(),
        )
    }
}

impl TrackWorkspaceAccessTime for StorageProvider<'_> {
    fn track_workspace_access_time(&self, workspace: Workspace) -> Result<Workspace> {
        self.track_workspace_access_time(workspace.id())?;

        GetWorkspace::get_workspace(self, workspace.id())
    }
}
