use chrono::Utc;
use hermione_internals::sqlite::{
    self, BackupCredentialsRecord, CommandRecord, ListCommandsQuery, ListWorkspacesQueryOptions,
    OptionalValue, UpdateCommandQueryOptions, UpdateWorkspaceQueryOptions, WorkspaceRecord,
};
use hermione_nexus::{
    definitions::{
        BackupCredentials, BackupProviderKind, Command, CommandId, Workspace, WorkspaceId,
    },
    services::{
        CreateCommand, CreateWorkspace, DeleteBackupCredentials, DeleteCommand, DeleteWorkspace,
        DeleteWorkspaceCommands, EditCommandParameters, EditWorkspaceParameters,
        FilterCommandsParameters, FilterWorkspacesParameters, FindBackupCredentials, FindCommand,
        FindWorkspace, ListBackupCredentials, ListCommands, ListWorkspaces, NewCommandParameters,
        NewWorkspaceParameters, SaveBackupCredentials, StorageService, TrackCommandExecuteTime,
        TrackWorkspaceAccessTime, UpdateCommand, UpdateWorkspace, UpsertCommands, UpsertWorkspaces,
    },
    Error, Result,
};
use rusqlite::Connection;
use uuid::Uuid;

pub struct Storage<'a> {
    conn: &'a Connection,
}

impl<'a> Storage<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }
}

fn internal_error(err: rusqlite::Error) -> Error {
    Error::storage(eyre::Error::new(err))
}

impl StorageService for Storage<'_> {}

impl CreateCommand for Storage<'_> {
    fn create_command(&self, parameters: NewCommandParameters) -> Result<Command> {
        let NewCommandParameters {
            name,
            program,
            workspace_id,
        } = parameters;

        let record = CommandRecord {
            id: Uuid::new_v4().into_bytes(),
            last_execute_time: None,
            name,
            program,
            workspace_id: workspace_id.into_bytes(),
        };

        let command = record.clone().try_into()?;

        sqlite::insert_command(self.conn, record).map_err(internal_error)?;

        Ok(command)
    }
}

impl CreateWorkspace for Storage<'_> {
    fn create_workspace(&self, parameters: NewWorkspaceParameters) -> Result<Workspace> {
        let NewWorkspaceParameters { name, location } = parameters;

        let record = WorkspaceRecord {
            id: Uuid::new_v4().into_bytes(),
            last_access_time: None,
            location,
            name,
        };

        let workspace = record.clone().try_into()?;

        sqlite::insert_workspace(self.conn, record).map_err(internal_error)?;

        Ok(workspace)
    }
}

impl DeleteBackupCredentials for Storage<'_> {
    fn delete_backup_credentials(&self, kind: BackupProviderKind) -> Result<()> {
        sqlite::delete_backup_credentials(self.conn, kind).map_err(internal_error)?;

        Ok(())
    }
}

impl DeleteCommand for Storage<'_> {
    fn delete_command(&self, id: CommandId) -> Result<()> {
        sqlite::delete_command(self.conn, id.as_bytes()).map_err(internal_error)?;

        Ok(())
    }
}

impl DeleteWorkspace for Storage<'_> {
    fn delete_workspace(&self, id: &WorkspaceId) -> Result<()> {
        sqlite::delete_workspace(self.conn, id.as_bytes()).map_err(internal_error)?;

        Ok(())
    }
}

impl DeleteWorkspaceCommands for Storage<'_> {
    fn delete_workspace_commands(&self, id: &WorkspaceId) -> Result<()> {
        sqlite::delete_workspace_commands(self.conn, id.as_bytes()).map_err(internal_error)?;

        Ok(())
    }
}

impl FindBackupCredentials for Storage<'_> {
    fn find_backup_credentials(
        &self,
        kind: BackupProviderKind,
    ) -> Result<Option<BackupCredentials>> {
        sqlite::find_backup_credentials(self.conn, kind)
            .map_err(internal_error)?
            .map(TryFrom::try_from)
            .transpose()
    }
}

impl FindCommand for Storage<'_> {
    fn find_command(&self, id: CommandId) -> Result<Option<Command>> {
        sqlite::find_command(self.conn, id.as_bytes())
            .map_err(internal_error)?
            .map(TryFrom::try_from)
            .transpose()
    }
}

impl FindWorkspace for Storage<'_> {
    fn find_workspace(&self, id: &WorkspaceId) -> Result<Option<Workspace>> {
        sqlite::find_workspace(self.conn, id.as_bytes())
            .map_err(internal_error)?
            .map(TryFrom::try_from)
            .transpose()
    }
}

impl ListBackupCredentials for Storage<'_> {
    fn list_backup_credentials(&self) -> Result<Vec<BackupCredentials>> {
        sqlite::list_backup_credentials(self.conn)
            .map_err(internal_error)?
            .into_iter()
            .map(TryFrom::try_from)
            .collect::<Result<Vec<_>>>()
    }
}

impl ListCommands for Storage<'_> {
    fn list_commands(&self, parameters: FilterCommandsParameters) -> Result<Vec<Command>> {
        let FilterCommandsParameters {
            program_contains,
            page_number,
            page_size,
            workspace_id,
        } = parameters;

        sqlite::list_commands(
            self.conn,
            ListCommandsQuery {
                program_contains: program_contains.unwrap_or_default(),
                workspace_id: workspace_id.map(|id| id.into_bytes()),
                offset: page_number,
                limit: page_size,
            },
        )
        .map_err(internal_error)?
        .into_iter()
        .map(TryFrom::try_from)
        .collect::<Result<Vec<_>>>()
    }
}

impl ListWorkspaces for Storage<'_> {
    fn list_workspaces(&self, parameters: FilterWorkspacesParameters) -> Result<Vec<Workspace>> {
        let FilterWorkspacesParameters {
            name_contains,
            page_number,
            page_size,
        } = parameters;

        sqlite::list_workspaces(
            self.conn,
            ListWorkspacesQueryOptions {
                name_contains: name_contains.unwrap_or_default(),
                limit: page_size,
                offset: page_number,
            },
        )
        .map_err(internal_error)?
        .into_iter()
        .map(TryFrom::try_from)
        .collect::<Result<Vec<_>>>()
    }
}

impl SaveBackupCredentials for Storage<'_> {
    fn save_backup_credentials(&self, credentials: &BackupCredentials) -> Result<()> {
        let kind = match credentials {
            BackupCredentials::Notion(_) => BackupProviderKind::Notion,
        };

        let record: BackupCredentialsRecord = credentials.try_into()?;

        let found = sqlite::find_backup_credentials(self.conn, kind).map_err(internal_error)?;

        if found.is_some() {
            sqlite::update_backup_credentials(self.conn, record)
        } else {
            sqlite::insert_backup_credentials(self.conn, record)
        }
        .map_err(internal_error)?;

        Ok(())
    }
}

impl TrackCommandExecuteTime for Storage<'_> {
    fn track_command_execute_time(&self, id: CommandId) -> Result<()> {
        sqlite::update_command(
            self.conn,
            UpdateCommandQueryOptions {
                id: id.into_bytes(),
                last_execute_time: Some(OptionalValue::Value(timestamp_micros())),
                name: None,
                program: None,
            },
        )
        .map_err(internal_error)?;

        Ok(())
    }
}

impl TrackWorkspaceAccessTime for Storage<'_> {
    fn track_workspace_access_time(&self, id: &WorkspaceId) -> Result<()> {
        sqlite::update_workspace(
            self.conn,
            UpdateWorkspaceQueryOptions {
                id: id.into_bytes(),
                last_access_time: Some(OptionalValue::Value(timestamp_micros())),
                location: None,
                name: None,
            },
        )
        .map_err(internal_error)?;

        Ok(())
    }
}

impl UpdateCommand for Storage<'_> {
    fn update_command(&self, parameters: EditCommandParameters) -> Result<()> {
        let EditCommandParameters { id, name, program } = parameters;

        sqlite::update_command(
            self.conn,
            UpdateCommandQueryOptions {
                id: id.into_bytes(),
                last_execute_time: None,
                name: Some(name.to_string()),
                program: Some(program.to_string()),
            },
        )
        .map_err(internal_error)?;

        Ok(())
    }
}

impl UpsertCommands for Storage<'_> {
    fn upsert_commands(&self, commands: Vec<Command>) -> Result<()> {
        let records = commands.into_iter().map(From::from).collect();

        sqlite::restore_commands(self.conn, records).map_err(internal_error)
    }
}

impl UpsertWorkspaces for Storage<'_> {
    fn upsert_workspaces(&self, workspaces: Vec<Workspace>) -> Result<()> {
        let records = workspaces.into_iter().map(From::from).collect();

        sqlite::restore_workspaces(self.conn, records).map_err(internal_error)
    }
}

impl UpdateWorkspace for Storage<'_> {
    fn update_workspace(&self, parameters: EditWorkspaceParameters) -> Result<()> {
        let EditWorkspaceParameters { id, location, name } = parameters;

        let location = location.map(ToString::to_string);

        sqlite::update_workspace(
            self.conn,
            UpdateWorkspaceQueryOptions {
                id: id.into_bytes(),
                last_access_time: None,
                location: Some(location.into()),
                name: Some(name.to_string()),
            },
        )
        .map_err(internal_error)?;

        Ok(())
    }
}

fn timestamp_micros() -> i64 {
    Utc::now().timestamp_micros()
}
