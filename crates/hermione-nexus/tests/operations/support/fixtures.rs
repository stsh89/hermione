use chrono::{DateTime, Utc};
use hermione_nexus::{
    definitions::{
        BackupCredentials, Command, CommandParameters, NotionBackupCredentialsParameters,
        Workspace, WorkspaceParameters,
    },
    Result,
};
use uuid::Uuid;

pub enum BackupCredentialsFixtureParameters {
    Notion(NotionBackupCredentialsFixtureParameters),
}

pub struct NotionBackupCredentialsFixtureParameters {
    pub api_key: Option<String>,
    pub workspaces_database_id: Option<String>,
    pub commands_database_id: Option<String>,
}

#[derive(Default)]
pub struct CommandFixtureParameters {
    pub name: Option<String>,
    pub program: Option<String>,
    pub last_execute_time: Option<DateTime<Utc>>,
    pub id: Option<Uuid>,
}

#[derive(Default)]
pub struct WorkspaceFixtureParameters {
    pub name: Option<String>,
    pub location: Option<String>,
    pub last_access_time: Option<DateTime<Utc>>,
    pub id: Option<Uuid>,
}

pub fn backup_credentials_fixture(
    parameters: BackupCredentialsFixtureParameters,
) -> BackupCredentials {
    match parameters {
        BackupCredentialsFixtureParameters::Notion(NotionBackupCredentialsFixtureParameters {
            api_key,
            workspaces_database_id,
            commands_database_id,
        }) => BackupCredentials::notion(NotionBackupCredentialsParameters {
            api_key: api_key.unwrap_or_default(),
            workspaces_database_id: workspaces_database_id.unwrap_or_default(),
            commands_database_id: commands_database_id.unwrap_or_default(),
        }),
    }
}

pub fn command_fixture(
    workspace: &Workspace,
    parameters: CommandFixtureParameters,
) -> Result<Command> {
    let CommandFixtureParameters {
        name,
        program,
        id,
        last_execute_time,
    } = parameters;

    Command::new(CommandParameters {
        id: id.unwrap_or(Uuid::new_v4()),
        last_execute_time,
        name: name.unwrap_or("Test command".to_string()),
        program: program.unwrap_or("ping 1.1.1.1".to_string()),
        workspace_id: workspace.id().clone(),
    })
}

pub fn workspace_fixture(parameters: WorkspaceFixtureParameters) -> Result<Workspace> {
    let WorkspaceFixtureParameters {
        name,
        location,
        last_access_time,
        id,
    } = parameters;

    Workspace::new(WorkspaceParameters {
        id: id.unwrap_or(Uuid::new_v4()),
        last_access_time,
        location,
        name: name.unwrap_or("Test workspace".to_string()),
    })
}
