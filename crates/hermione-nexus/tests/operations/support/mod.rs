mod backup;
mod clipboard;
mod fixtures;
mod notion_storage;
mod storage;
mod system;

use chrono::NaiveDateTime;
use hermione_nexus::definitions::{
    BackupCredentials, NotionBackupCredentialsParameters, Workspace, WorkspaceParameters,
};
use serde_json::Value as Json;

pub use backup::*;
pub use clipboard::*;
pub use fixtures::*;
pub use notion_storage::*;
pub use storage::*;
pub use system::*;

use uuid::Uuid;

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
    let credentials = BackupCredentials::notion(NotionBackupCredentialsParameters {
        api_key: parameters["api_key"].to_string(),
        commands_database_id: parameters["commands_database_id"].to_string(),
        workspaces_database_id: parameters["workspaces_database_id"].to_string(),
    });

    storage
        .backup_credentials
        .write()
        .expect("Should be able to insert backup credentials")
        .insert(NOTION_CREDENTIALS_KEY.to_string(), credentials);
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

pub fn insert_notion_workspace(storage: &MockNotionStorage, parameters: Json) {
    let external_id = parameters["external_id"]
        .as_str()
        .expect("Insert notion workspace parameters should have `external_id` key")
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
