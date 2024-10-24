use crate::database::{CommandRecord, DatabaseProvider, WorkspaceRecord};
use chrono::Utc;
use hermione_ops::{
    commands::{Command, CommandWorkspaceScopedId, GetCommandFromWorkspace},
    extensions::{TrackCommandExecutionTime, TrackWorkspaceAccessTime},
    workspaces::{GetWorkspace, Workspace},
    Error,
};
use rusqlite::params;
use uuid::Uuid;

impl TrackCommandExecutionTime for DatabaseProvider {
    fn track_command_execution_time(&self, command: Command) -> Result<Command, Error> {
        let record = CommandRecord::from_entity(&command)?;

        let last_execute_time = Utc::now()
            .timestamp_nanos_opt()
            .ok_or(eyre::eyre!("Failed to get timestamp"))?;

        let mut statement = self
            .connection()
            .prepare(
                "UPDATE commands
                SET last_execute_time = ?1
                WHERE id = ?2 AND workspace_id = ?3",
            )
            .map_err(eyre::Error::new)?;

        statement
            .execute(params![last_execute_time, record.id, record.workspace_id])
            .map_err(eyre::Error::new)?;

        self.get_command_from_workspace(CommandWorkspaceScopedId {
            command_id: Uuid::from_bytes(record.id),
            workspace_id: Uuid::from_bytes(record.workspace_id),
        })
    }
}

impl TrackWorkspaceAccessTime for DatabaseProvider {
    fn track_access_time(&self, workspace: Workspace) -> Result<Workspace, Error> {
        let record: WorkspaceRecord = workspace.try_into()?;

        let last_access_time = Utc::now()
            .timestamp_nanos_opt()
            .ok_or(eyre::eyre!("Failed to get timestamp"))?;

        let mut statement = self
            .connection()
            .prepare(
                "UPDATE workspaces
                SET last_access_time = ?1
                WHERE id = ?2",
            )
            .map_err(eyre::Error::new)?;

        statement
            .execute(params![last_access_time, record.id])
            .map_err(eyre::Error::new)?;

        self.get_workspace(Uuid::from_bytes(record.id))
    }
}
