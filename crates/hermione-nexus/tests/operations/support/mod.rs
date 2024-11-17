mod backup;
mod clipboard;
mod storage;
mod system;

use chrono::NaiveDateTime;
use hermione_nexus::definitions::{
    BackupCredentials, Command, CommandParameters, NotionBackupCredentialsParameters, Workspace,
    WorkspaceParameters,
};
use serde_json::Value as Json;
use uuid::Uuid;

pub use backup::*;
pub use clipboard::*;
pub use storage::*;
pub use system::*;

pub fn assert_command(command: &Command, parameters: Json) {
    let name = parameters["name"]
        .as_str()
        .expect("Assert command parameters should have `name` key")
        .to_string();

    let program = parameters["program"]
        .as_str()
        .expect("Assert command parameters should have `program` key")
        .to_string();

    let last_execute_time = parameters["last_execute_time"].as_str().map(|value| {
        NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
            .expect("Assert command parameters should have valid `last_execute_time` value")
            .and_utc()
    });

    let workspace_id = parameters["workspace_id"]
        .as_str()
        .expect("Assert command parameters should have `workspace_id` key")
        .to_string();

    assert_eq!(command.name(), name);
    assert_eq!(command.program(), program);
    assert_eq!(command.last_execute_time(), last_execute_time.as_ref());
    assert_eq!(command.workspace_id().to_string(), workspace_id);
}

pub fn assert_notion_backup_credentials(backup_credentials: &BackupCredentials, parameters: Json) {
    let api_key = parameters["api_key"].as_str().unwrap().to_string();

    let commands_database_id = parameters["commands_database_id"]
        .as_str()
        .unwrap()
        .to_string();

    let workspaces_database_id = parameters["workspaces_database_id"]
        .as_str()
        .unwrap()
        .to_string();

    let BackupCredentials::Notion(backup_credentials) = backup_credentials;

    assert_eq!(backup_credentials.api_key(), api_key);

    assert_eq!(
        backup_credentials.commands_database_id(),
        commands_database_id
    );

    assert_eq!(
        backup_credentials.workspaces_database_id(),
        workspaces_database_id
    );
}

pub fn assert_workspace(workspace: &Workspace, parameters: Json) {
    let location = parameters["location"].as_str();

    let name = parameters["name"]
        .as_str()
        .expect("Assert workspace parameters should have `name` key")
        .to_string();

    let last_access_time = parameters["last_access_time"].as_str().map(|value| {
        NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
            .expect("Assert workspace parameters should have valid `last_access_time` value")
            .and_utc()
    });

    assert_eq!(workspace.name(), name);
    assert_eq!(workspace.location(), location);
    assert_eq!(workspace.last_access_time(), last_access_time.as_ref());
}

pub fn get_clipboard_content(clipboard: &MockClipboard) -> String {
    clipboard
        .content
        .read()
        .expect("Should be able to get clipboard content")
        .clone()
        .unwrap_or_default()
}

pub fn get_command(storage: &InMemoryStorage, id: Uuid) -> Command {
    storage
        .commands
        .read()
        .expect("Should be able to get command")
        .get(&id)
        .cloned()
        .unwrap_or_else(|| panic!("Command {} should exist", id.braced()))
}

pub fn get_notion_backup_credentials(storage: &InMemoryStorage) -> BackupCredentials {
    storage
        .backup_credentials
        .read()
        .expect("Should be able to get Notion backup credentials")
        .get(NOTION_CREDENTIALS_KEY)
        .cloned()
        .unwrap_or_else(|| panic!("Notion backup credentials should exist"))
}

pub fn get_workspace(storage: &InMemoryStorage, id: Uuid) -> Workspace {
    storage
        .workspaces
        .read()
        .expect("Should be able to get workspace")
        .get(&id)
        .cloned()
        .unwrap_or_else(|| panic!("Workspace {} should exist", id.braced()))
}

pub fn insert_notion_backup_credentials(storage: &InMemoryStorage, parameters: Json) {
    let api_key = parameters["api_key"]
        .as_str()
        .expect("Insert Notion credentials parameters should have `api_key` key")
        .to_string();

    let commands_database_id = parameters["commands_database_id"]
        .as_str()
        .expect("Insert Notion credentials parameters should have `commands_database_id` key")
        .to_string();

    let workspaces_database_id = parameters["workspaces_database_id"]
        .as_str()
        .expect("Insert Notion credentials parameters should have `workspaces_database_id` key")
        .to_string();

    let credentials = BackupCredentials::notion(NotionBackupCredentialsParameters {
        api_key,
        commands_database_id,
        workspaces_database_id,
    });

    storage
        .backup_credentials
        .write()
        .expect("Should be able to insert Notion backup credentials")
        .insert(NOTION_CREDENTIALS_KEY.to_string(), credentials);
}

pub fn insert_command(storage: &InMemoryStorage, parameters: Json) {
    let id = parameters["id"]
        .as_str()
        .expect("Insert command parameters should have `id` key")
        .parse()
        .expect("Insert command parameters should have valid `id` value");

    let last_execute_time = parameters["last_execute_time"].as_str().map(|value| {
        NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
            .expect("Insert command parameters should have valid `last_execute_time` value")
            .and_utc()
    });

    let name = parameters["name"]
        .as_str()
        .map(ToString::to_string)
        .unwrap_or_default();

    let program = parameters["program"]
        .as_str()
        .map(ToString::to_string)
        .unwrap_or_default();

    let workspace_id: Uuid = parameters["workspace_id"]
        .as_str()
        .expect("Insert command parameters should have `workspace_id` key")
        .parse()
        .expect("Insert command parameters should have valid `workspace_id` value");

    let command = Command::new(CommandParameters {
        id,
        name,
        program,
        last_execute_time,
        workspace_id: workspace_id.into(),
    })
    .expect("Command should be valid");

    storage
        .commands
        .write()
        .expect("Should be able to insert command")
        .insert(**command.id(), command);
}

pub fn insert_workspace(storage: &InMemoryStorage, parameters: Json) {
    let id = parameters["id"]
        .as_str()
        .expect("Insert workspace parameters should have `id` key")
        .parse()
        .expect("Insert workspace parameters should have valid `id` value");

    let last_access_time = parameters["last_access_time"].as_str().map(|value| {
        NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
            .expect("Insert workspace parameters should have valid `last_access_time` value")
            .and_utc()
    });

    let name = parameters["name"]
        .as_str()
        .map(ToString::to_string)
        .unwrap_or_default();

    let location = parameters["location"].as_str().map(ToString::to_string);

    let workspace = Workspace::new(WorkspaceParameters {
        id,
        name,
        location,
        last_access_time,
    })
    .expect("Workspace should be valid");

    storage
        .workspaces
        .write()
        .expect("Should be able to insert workspace")
        .insert(**workspace.id(), workspace);
}

pub fn insert_notion_command(storage: &MockNotionStorage, parameters: Json) {
    let external_id = parameters["external_id"]
        .as_str()
        .expect("Insert Notion command parameters should have `external_id` key")
        .to_string();

    let name = parameters["name"]
        .as_str()
        .expect("Insert Notion command parameters should have `name` key")
        .to_string();

    let program = parameters["program"]
        .as_str()
        .expect("Insert Notion command parameters should have `program` key")
        .to_string();

    let workspace_id = parameters["workspace_id"]
        .as_str()
        .expect("Insert Notion command parameters should have `workspace_id` key")
        .to_string();

    let entry = MockNotionCommandEntry {
        external_id: external_id.clone(),
        name,
        program,
        workspace_id,
    };

    storage
        .commands
        .write()
        .expect("Should be able to insert Notion command")
        .insert(external_id, entry);
}

pub fn insert_notion_workspace(storage: &MockNotionStorage, parameters: Json) {
    let external_id = parameters["external_id"]
        .as_str()
        .expect("Insert Notion workspace parameters should have `external_id` key")
        .to_string();

    let name = parameters["name"]
        .as_str()
        .expect("Insert Notion workspace parameters should have `name` key")
        .to_string();

    let location = parameters["location"]
        .as_str()
        .map(ToString::to_string)
        .unwrap_or_default();

    let entry = MockNotionWorkspaceEntry {
        external_id: external_id.clone(),
        name,
        location,
    };

    storage
        .workspaces
        .write()
        .expect("Should be able to insert Notion workspace")
        .insert(external_id, entry);
}

pub fn list_backup_credentials(storage: &InMemoryStorage) -> Vec<BackupCredentials> {
    storage
        .backup_credentials
        .read()
        .expect("Should be able to list backup credentials")
        .values()
        .cloned()
        .collect()
}
