#[macro_use]
pub mod table;

mod backup;
mod storage;
mod system;

pub use backup::*;
use chrono::{DateTime, Utc};
pub use storage::*;
pub use system::*;

use hermione_nexus::definitions::{
    BackupCredentials, BackupProviderKind, Command, CommandId, CommandParameters,
    NotionBackupCredentialsParameters, Workspace, WorkspaceId, WorkspaceParameters,
};
use table::Table;

pub fn assert_clipboard_content(system: &MockSystem, expected_clipboard_content: &str) {
    let content = get_clipboard_content(system);

    assert_eq!(content.as_deref(), Some(expected_clipboard_content));
}

pub fn assert_command(command: &Command, parameters: &Table) {
    let name = parameters.get_text("name");
    let program = parameters.get_text("program");
    let last_execute_time = parameters.maybe_get_date_time("last_execute_time");
    let workspace_id = parameters.get_workspace_id("workspace_id");

    assert_eq!(command.name(), name);
    assert_eq!(command.program(), program);
    assert_eq!(command.last_execute_time(), last_execute_time.as_ref());
    assert_eq!(command.workspace_id(), workspace_id);
}

pub fn assert_backup_credentials_count(storage: &InMemoryStorage, expected_count: usize) {
    let count = count_backup_credentials(storage);

    assert_eq!(count, expected_count);
}

pub fn assert_commands_count(storage: &InMemoryStorage, expected_count: usize) {
    let count = count_commands(storage);

    assert_eq!(count, expected_count);
}

pub fn assert_file_system_location(system: &MockSystem, expected_location: &str) {
    let location = get_file_system_location(system);

    assert_eq!(location.as_deref(), Some(expected_location));
}

pub fn assert_last_executed_program(system: &MockSystem, expected_program: &str) {
    let program = get_last_executed_program(system);

    assert_eq!(program.as_deref(), Some(expected_program));
}

pub fn assert_notion_backup_credentials(backup_credentials: &BackupCredentials, parameters: Table) {
    let api_key = parameters.get_text("api_key");
    let commands_database_id = parameters.get_text("commands_database_id");
    let workspaces_database_id = parameters.get_text("workspaces_database_id");

    match backup_credentials {
        BackupCredentials::Notion(credentials) => {
            assert_eq!(credentials.api_key(), api_key);
            assert_eq!(credentials.commands_database_id(), commands_database_id);
            assert_eq!(credentials.workspaces_database_id(), workspaces_database_id);
        }
    }
}

pub fn assert_notion_command(notion_command: &NotionCommand, parameters: Table) {
    let name = parameters.get_text("name");
    let program = parameters.get_text("program");
    let workspace_id = parameters.get_text("workspace_id");

    assert_eq!(&notion_command.name, name);
    assert_eq!(&notion_command.program, program);
    assert_eq!(&notion_command.workspace_id, workspace_id);
}

pub fn assert_notion_workspace(notion_workspace: &NotionWorkspace, parameters: Table) {
    let location = parameters.get_text("location");
    let name = parameters.get_text("name");

    assert_eq!(&notion_workspace.location, location);
    assert_eq!(&notion_workspace.name, name);
}

pub fn assert_workspace(workspace: &Workspace, parameters: &Table) {
    let last_access_time = parameters.maybe_get_date_time("last_access_time");
    let location = parameters.maybe_get_text("location");
    let name = parameters.get_text("name");

    assert_eq!(workspace.last_access_time(), last_access_time.as_ref());
    assert_eq!(workspace.location(), location);
    assert_eq!(workspace.name(), name);
}

pub fn assert_workspaces_count(storage: &InMemoryStorage, expected_count: usize) {
    let count = count_workspaces(storage);

    assert_eq!(count, expected_count);
}

pub fn count_backup_credentials(storage: &InMemoryStorage) -> usize {
    storage
        .backup_credentials
        .read()
        .expect("Should be able to get backup credentials")
        .len()
}

pub fn count_commands(storage: &InMemoryStorage) -> usize {
    storage.commands.read().unwrap().len()
}

pub fn count_workspaces(storage: &InMemoryStorage) -> usize {
    storage.workspaces.read().unwrap().len()
}

pub fn get_backup_provider_kind(name: &str) -> BackupProviderKind {
    match name {
        "Notion" => BackupProviderKind::Notion,
        name => panic!("Could not find backup provider with name: {}", name),
    }
}

pub fn get_clipboard_content(system: &MockSystem) -> Option<String> {
    system
        .clipboard
        .read()
        .expect("Should be able to get clipboard content")
        .clone()
}

pub fn get_command(storage: &InMemoryStorage, id: CommandId) -> Command {
    storage
        .commands
        .read()
        .expect("Should be able to get command")
        .get(&id)
        .cloned()
        .unwrap_or_else(|| panic!("Command {} should exist", id))
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

pub fn get_notion_command(notion: &MockNotionStorage, external_id: &str) -> NotionCommand {
    notion
        .commands
        .read()
        .expect("Should be able to obtain read access to Notion commands")
        .get(external_id)
        .unwrap_or_else(|| panic!("Could not find Notion command with ID: {}", external_id))
        .clone()
}

pub fn get_notion_workspace(notion: &MockNotionStorage, external_id: &str) -> NotionWorkspace {
    notion
        .workspaces
        .read()
        .expect("Should be able to obtain read access to Notion workspaces")
        .get(external_id)
        .unwrap_or_else(|| panic!("Could not find Notion workspace with ID: {}", external_id))
        .clone()
}

pub fn get_file_system_location(system: &MockSystem) -> Option<String> {
    system
        .location
        .read()
        .expect("Should be able to access system location")
        .clone()
}

pub fn get_last_executed_program(system: &MockSystem) -> Option<String> {
    system
        .program
        .read()
        .expect("Should be able to get system program")
        .clone()
}

pub fn get_workspace(storage: &InMemoryStorage, id: WorkspaceId) -> Workspace {
    storage
        .workspaces
        .read()
        .expect("Should be able to get workspace")
        .get(&id)
        .cloned()
        .unwrap_or_else(|| panic!("Workspace {} should exist", id))
}

pub fn insert_command(storage: &InMemoryStorage, parameters: Table) {
    let id = parameters.get_uuid("id");
    let last_execute_time = parameters.maybe_get_date_time("last_execute_time");
    let name = parameters.get_text("name");
    let program = parameters.get_text("program");
    let workspace_id = parameters.get_workspace_id("workspace_id");

    let command = Command::new(CommandParameters {
        id,
        name: name.to_string(),
        program: program.to_string(),
        last_execute_time,
        workspace_id,
    })
    .expect("Command should be valid");

    storage
        .commands
        .write()
        .expect("Should be able to insert command")
        .insert(command.id(), command);
}

pub fn insert_notion_backup_credentials(storage: &InMemoryStorage, parameters: Table) {
    let api_key = parameters.get_text("api_key");
    let commands_database_id = parameters.get_text("commands_database_id");
    let workspaces_database_id = parameters.get_text("workspaces_database_id");

    let credentials = BackupCredentials::notion(NotionBackupCredentialsParameters {
        api_key: api_key.to_string(),
        commands_database_id: commands_database_id.to_string(),
        workspaces_database_id: workspaces_database_id.to_string(),
    });

    storage
        .backup_credentials
        .write()
        .expect("Should be able to insert Notion backup credentials")
        .insert(NOTION_CREDENTIALS_KEY.to_string(), credentials);
}

pub fn insert_notion_command(storage: &MockNotionStorage, parameters: Table) {
    let external_id = parameters.get_text("external_id");
    let name = parameters.get_text("name");
    let program = parameters.get_text("program");
    let workspace_id = parameters.get_text("workspace_id");

    let entry = NotionCommand {
        external_id: external_id.to_string(),
        name: name.to_string(),
        program: program.to_string(),
        workspace_id: workspace_id.to_string(),
    };

    storage
        .commands
        .write()
        .expect("Should be able to insert Notion command entry")
        .insert(external_id.to_string(), entry);
}

pub fn insert_notion_workspace(storage: &MockNotionStorage, parameters: Table) {
    let external_id = parameters.get_text("external_id");
    let name = parameters.get_text("name");
    let location = parameters.get_text("location");

    let entry = NotionWorkspace {
        external_id: external_id.to_string(),
        name: name.to_string(),
        location: location.to_string(),
    };

    storage
        .workspaces
        .write()
        .expect("Should be able to insert Notion workspace entry")
        .insert(external_id.to_string(), entry);
}

pub fn insert_workspace(storage: &InMemoryStorage, parameters: Table) {
    let id = parameters.get_uuid("id");
    let last_access_time = parameters.maybe_get_date_time("last_access_time");
    let name = parameters.get_text("name");
    let location = parameters.maybe_get_text("location");

    let workspace = Workspace::new(WorkspaceParameters {
        id,
        name: name.to_string(),
        location: location.map(ToString::to_string),
        last_access_time,
    })
    .expect("Workspace should be valid");

    storage
        .workspaces
        .write()
        .expect("Should be able to insert workspace")
        .insert(workspace.id(), workspace);
}

pub fn freeze_storage_time(storage: &InMemoryStorage, time: DateTime<Utc>) {
    let mut timestamp = storage
        .now
        .write()
        .expect("Should be able to freeze storage time");

    *timestamp = Some(time);
}
