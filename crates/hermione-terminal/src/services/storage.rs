use chrono::{DateTime, Utc};
use hermione_drive::sqlite::{
    self, BackupCredentialsRecord, CommandRecord, ListCommandsQuery, ListWorkspacesQuery,
    WorkspaceRecord,
};
use hermione_nexus::{
    definitions::{
        BackupCredentials, BackupProviderKind, Command, CommandId, CommandParameters,
        NotionBackupCredentialsParameters, Workspace, WorkspaceId, WorkspaceParameters,
    },
    services::{
        CreateCommand, CreateWorkspace, DeleteCommand, DeleteWorkspace, DeleteWorkspaceCommands,
        EditCommandParameters, EditWorkspaceParameters, FilterCommandsParameters,
        FilterWorkspacesParameters, FindBackupCredentials, FindCommand, FindWorkspace,
        ListBackupCredentials, ListCommands, ListWorkspaces, NewCommandParameters,
        NewWorkspaceParameters, SaveBackupCredentials, StorageService, TrackCommandExecuteTime,
        TrackWorkspaceAccessTime, UpdateCommand, UpdateWorkspace,
    },
    Error, Result,
};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const NOTION_BACKUP_CREDENTIALS_ID: &str = "Notion";

pub struct Storage<'a> {
    pub conn: &'a Connection,
}

fn internal_error(err: rusqlite::Error) -> Error {
    Error::Storage(eyre::Error::new(err))
}

#[derive(Serialize, Deserialize)]
struct NotionBackupSecrets {
    api_key: String,
    commands_database_id: String,
    workspaces_database_id: String,
}

enum BackupCredentialsId {
    Notion,
}

fn backup_credentials_from_record(record: BackupCredentialsRecord) -> Result<BackupCredentials> {
    let BackupCredentialsRecord { id, secrets } = record;

    match BackupCredentialsId::try_from(id.as_str())? {
        BackupCredentialsId::Notion => {
            let secrets: NotionBackupSecrets = serde_json::from_str(&secrets)
                .map_err(|err| Error::Storage(eyre::Error::new(err)))?;

            let NotionBackupSecrets {
                api_key,
                commands_database_id,
                workspaces_database_id,
            } = secrets;

            Ok(BackupCredentials::notion(
                NotionBackupCredentialsParameters {
                    api_key,
                    commands_database_id,
                    workspaces_database_id,
                },
            ))
        }
    }
}

fn command_from_record(record: CommandRecord) -> Result<Command> {
    let CommandRecord {
        id,
        last_execute_time,
        name,
        program,
        workspace_id,
    } = record;

    Command::new(CommandParameters {
        id: Uuid::from_bytes(id),
        last_execute_time: last_execute_time.and_then(DateTime::from_timestamp_micros),
        name,
        program,
        workspace_id: Uuid::from_bytes(workspace_id).into(),
    })
}

fn workspace_from_record(record: WorkspaceRecord) -> Result<Workspace> {
    let WorkspaceRecord {
        id,
        last_access_time,
        location,
        name,
    } = record;

    Workspace::new(WorkspaceParameters {
        id: Uuid::from_bytes(id),
        last_access_time: last_access_time.and_then(DateTime::from_timestamp_micros),
        location,
        name,
    })
}

impl TryFrom<&str> for BackupCredentialsId {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let id = match value {
            NOTION_BACKUP_CREDENTIALS_ID => BackupCredentialsId::Notion,
            _ => {
                return Err(Error::Storage(eyre::Error::msg(format!(
                    "Unexpected backup credentials id: {}",
                    value
                ))))
            }
        };

        Ok(id)
    }
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

        let command = command_from_record(record.clone())?;

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

        let workspace = workspace_from_record(record.clone())?;

        sqlite::insert_workspace(self.conn, record).map_err(internal_error)?;

        Ok(workspace)
    }
}

impl DeleteCommand for Storage<'_> {
    fn delete_command(&self, id: &CommandId) -> Result<()> {
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
        kind: &BackupProviderKind,
    ) -> Result<Option<BackupCredentials>> {
        let id = match kind {
            BackupProviderKind::Notion => NOTION_BACKUP_CREDENTIALS_ID,
            BackupProviderKind::Unknown => return Ok(None),
        };

        let record = sqlite::find_backup_credentials(self.conn, id).map_err(internal_error)?;

        record.map(backup_credentials_from_record).transpose()
    }
}

impl FindCommand for Storage<'_> {
    fn find_command(&self, id: &CommandId) -> Result<Option<Command>> {
        let record = sqlite::find_command(self.conn, id.as_bytes()).map_err(internal_error)?;

        record.map(command_from_record).transpose()
    }
}

impl FindWorkspace for Storage<'_> {
    fn find_workspace(&self, id: &WorkspaceId) -> Result<Option<Workspace>> {
        let record = sqlite::find_workspace(self.conn, id.as_bytes()).map_err(internal_error)?;

        record.map(workspace_from_record).transpose()
    }
}

impl ListBackupCredentials for Storage<'_> {
    fn list_backup_credentials(&self) -> Result<Vec<BackupCredentials>> {
        let records = sqlite::list_backup_credentials(self.conn).map_err(internal_error)?;

        records
            .into_iter()
            .map(backup_credentials_from_record)
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

        let records = sqlite::list_commands(
            self.conn,
            ListCommandsQuery {
                program_contains: program_contains.unwrap_or_default(),
                workspace_id: workspace_id.map(|id| id.into_bytes()),
                offset: page_number,
                limit: page_size,
            },
        )
        .map_err(internal_error)?;

        records
            .into_iter()
            .map(command_from_record)
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

        let records = sqlite::list_workspaces(
            self.conn,
            ListWorkspacesQuery {
                name_contains: name_contains.unwrap_or_default(),
                limit: page_size,
                offset: page_number,
            },
        )
        .map_err(internal_error)?;

        records
            .into_iter()
            .map(workspace_from_record)
            .collect::<Result<Vec<_>>>()
    }
}

impl SaveBackupCredentials for Storage<'_> {
    fn save_backup_credentials(&self, credentials: &BackupCredentials) -> Result<()> {
        let record = match credentials {
            BackupCredentials::Notion(notion_backup_credentials) => BackupCredentialsRecord {
                id: NOTION_BACKUP_CREDENTIALS_ID.to_string(),
                secrets: serde_json::to_string(&NotionBackupSecrets {
                    api_key: notion_backup_credentials.api_key().to_string(),
                    commands_database_id: notion_backup_credentials
                        .commands_database_id()
                        .to_string(),
                    workspaces_database_id: notion_backup_credentials
                        .workspaces_database_id()
                        .to_string(),
                })
                .map_err(|err| Error::Storage(eyre::Error::new(err)))?,
            },
        };

        sqlite::insert_backup_credentials(self.conn, record).map_err(internal_error)?;

        Ok(())
    }
}

impl TrackCommandExecuteTime for Storage<'_> {
    fn track_command_execute_time(&self, id: &CommandId) -> Result<()> {
        sqlite::refresh_command_execute_time(
            self.conn,
            id.as_bytes(),
            Utc::now().timestamp_micros(),
        )
        .map_err(internal_error)?;

        Ok(())
    }
}

impl TrackWorkspaceAccessTime for Storage<'_> {
    fn track_workspace_access_time(&self, id: &WorkspaceId) -> Result<()> {
        sqlite::refresh_workspace_access_time(
            self.conn,
            id.as_bytes(),
            Utc::now().timestamp_micros(),
        )
        .map_err(internal_error)?;

        Ok(())
    }
}

impl UpdateCommand for Storage<'_> {
    fn update_command(&self, parameters: EditCommandParameters) -> Result<()> {
        let EditCommandParameters { id, name, program } = parameters;

        let record = sqlite::find_command(self.conn, id.as_bytes())
            .map_err(internal_error)?
            .ok_or(Error::NotFound(format!("Command {}", Uuid::nil().braced())))?;

        sqlite::update_command(
            self.conn,
            CommandRecord {
                name: name.to_string(),
                program: program.to_string(),
                ..record
            },
        )
        .map_err(internal_error)?;

        Ok(())
    }
}

impl UpdateWorkspace for Storage<'_> {
    fn update_workspace(&self, parameters: EditWorkspaceParameters) -> Result<()> {
        let EditWorkspaceParameters { id, location, name } = parameters;

        let record = sqlite::find_workspace(self.conn, id.as_bytes())
            .map_err(internal_error)?
            .ok_or(Error::NotFound(format!(
                "Workspace {}",
                Uuid::nil().braced()
            )))?;

        sqlite::update_workspace(
            self.conn,
            WorkspaceRecord {
                location: location.map(ToString::to_string),
                name: name.to_string(),
                ..record
            },
        )
        .map_err(internal_error)?;

        Ok(())
    }
}
