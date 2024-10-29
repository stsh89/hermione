use crate::{StorageProvider, StorageProviderResult};
use chrono::Utc;
use hermione_ops::{
    commands::{Command, CommandWorkspaceScopedId, GetCommandFromWorkspace},
    extensions::{TrackCommandExecutionTime, TrackWorkspaceAccessTime},
    workspaces::{GetWorkspace, Workspace},
    Result,
};
use rusqlite::params;
use uuid::Uuid;

impl<'a> StorageProvider<'a> {
    fn track_command_execution_time(
        &self,
        workspace_id: Uuid,
        command_id: Uuid,
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
                command_id.as_bytes(),
                workspace_id.as_bytes()
            ])?;

        Ok(())
    }

    fn track_workspace_access_time(&self, workspace_id: Uuid) -> StorageProviderResult<()> {
        let last_access_time = Utc::now().timestamp_nanos_opt();

        let mut statement = self.connection.prepare(
            "UPDATE workspaces
            SET last_access_time = ?1
            WHERE id = ?2",
        )?;

        statement.execute(params![last_access_time, workspace_id.as_bytes()])?;

        Ok(())
    }
}

impl TrackCommandExecutionTime for StorageProvider<'_> {
    fn track_command_execution_time(&self, command: Command) -> Result<Command> {
        let command_id = command.try_id()?;

        self.track_command_execution_time(command.workspace_id(), command_id)?;

        GetCommandFromWorkspace::get_command_from_workspace(
            self,
            CommandWorkspaceScopedId {
                command_id,
                workspace_id: command.workspace_id(),
            },
        )
    }
}

impl TrackWorkspaceAccessTime for StorageProvider<'_> {
    fn track_workspace_access_time(&self, workspace: Workspace) -> Result<Workspace> {
        let workspace_id = workspace.try_id()?;

        self.track_workspace_access_time(workspace_id)?;

        GetWorkspace::get_workspace(self, workspace_id)
    }
}
