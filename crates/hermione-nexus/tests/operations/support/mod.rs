#[macro_use]
pub mod table;

mod backup;
mod storage;
mod system;

pub use backup::*;
pub use storage::*;
pub use system::*;

use chrono::{DateTime, NaiveDateTime, Utc};
use hermione_nexus::definitions::{
    BackupCredentials, BackupProviderKind, Command, CommandId, CommandParameters,
    NotionBackupCredentialsParameters, Workspace, WorkspaceId, WorkspaceParameters,
};
use table::Table;
use uuid::Uuid;

pub struct ExistingCommand<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub program: &'a str,
    pub last_execute_time: Option<&'a str>,
    pub workspace_id: &'a str,
}

pub struct ExistingWorkspace<'a> {
    pub id: &'a str,
    pub last_access_time: Option<&'a str>,
    pub location: Option<&'a str>,
    pub name: &'a str,
}

pub struct ExpectedCommand<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub program: &'a str,
    pub last_execute_time: Option<&'a str>,
    pub workspace_id: &'a str,
}

pub struct ExpectedWorkspace<'a> {
    pub id: &'a str,
    pub last_access_time: Option<&'a str>,
    pub location: Option<&'a str>,
    pub name: &'a str,
}

impl<'a> ExpectedCommand<'a> {
    pub fn id(&self) -> CommandId {
        parse_command_id(self.id)
    }
}

impl<'a> ExpectedWorkspace<'a> {
    pub fn id(&self) -> WorkspaceId {
        parse_workspace_id(self.id)
    }
}

pub fn assert_system_location(system: &MockSystem, expected: &str) {
    let location = system.location.read().unwrap().clone();

    assert_eq!(location.as_deref(), Some(expected));
}

pub fn assert_workspaces(workspaces: Vec<Workspace>, expected_workspaces: Vec<ExpectedWorkspace>) {
    assert_eq!(workspaces.len(), expected_workspaces.len());

    for (index, expected_workspace) in expected_workspaces.into_iter().enumerate() {
        assert_workspace_new(workspaces[index].clone(), expected_workspace);
    }
}

pub fn assert_command_new(command: Command, expected: ExpectedCommand) {
    let expected = Command::from(expected);

    assert_eq!(command.id(), expected.id());
    assert_eq!(command.name(), expected.name());
    assert_eq!(command.program(), expected.program());
    assert_eq!(command.last_execute_time(), expected.last_execute_time(),);
}

pub fn assert_workspace_new(workspace: Workspace, expected: ExpectedWorkspace) {
    let expected = Workspace::from(expected);

    assert_eq!(workspace.id(), expected.id());
    assert_eq!(workspace.name(), expected.name());
    assert_eq!(workspace.location(), expected.location());
    assert_eq!(workspace.last_access_time(), expected.last_access_time(),);
}

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
    system.clipboard.read().unwrap().clone()
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

pub fn get_command(storage: &InMemoryStorage, id: CommandId) -> Command {
    maybe_get_command(storage, id).unwrap_or_else(|| panic!("Command {} should exist", id))
}

pub fn maybe_get_command(storage: &InMemoryStorage, id: CommandId) -> Option<Command> {
    storage.commands.read().unwrap().get(&id).cloned()
}

pub fn get_workspace(storage: &InMemoryStorage, id: WorkspaceId) -> Workspace {
    maybe_get_workspace(storage, id).unwrap_or_else(|| panic!("Workspace {} should exist", id))
}

pub fn maybe_get_workspace(storage: &InMemoryStorage, id: WorkspaceId) -> Option<Workspace> {
    storage.workspaces.read().unwrap().get(&id).cloned()
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

pub fn insert_workspace_new(storage: &InMemoryStorage, existing: ExistingWorkspace) {
    let workspace = Workspace::from(existing);

    storage
        .workspaces
        .write()
        .unwrap()
        .insert(workspace.id(), workspace);
}

pub fn insert_command_new(storage: &InMemoryStorage, existing: ExistingCommand) {
    let command = Command::from(existing);

    storage
        .commands
        .write()
        .unwrap()
        .insert(command.id(), command);
}

pub fn insert_workspaces(storage: &InMemoryStorage, workspaces: Vec<ExistingWorkspace>) {
    for workspace in workspaces {
        insert_workspace_new(storage, workspace);
    }
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
        .unwrap()
        .insert(workspace.id(), workspace);
}

pub fn freeze_storage_time(storage: &InMemoryStorage, time: DateTime<Utc>) {
    let mut timestamp = storage
        .now
        .write()
        .expect("Should be able to freeze storage time");

    *timestamp = Some(time);
}

pub fn maybe_parse_time(value: Option<&str>) -> Option<DateTime<Utc>> {
    value.map(parse_time)
}

pub fn parse_time(value: &str) -> DateTime<Utc> {
    NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
        .unwrap()
        .and_utc()
}

pub fn parse_uuid(value: &str) -> Uuid {
    Uuid::parse_str(value).unwrap()
}

pub fn parse_command_id(value: &str) -> CommandId {
    CommandId::parse_str(value).unwrap()
}

pub fn parse_workspace_id(value: &str) -> WorkspaceId {
    WorkspaceId::parse_str(value).unwrap()
}

impl<'a> From<ExistingCommand<'a>> for Command {
    fn from(value: ExistingCommand) -> Self {
        let ExistingCommand {
            id,
            name,
            program,
            workspace_id,
            last_execute_time,
        } = value;

        Command::new(CommandParameters {
            id: parse_uuid(id),
            name: name.to_string(),
            program: program.to_string(),
            workspace_id: parse_workspace_id(workspace_id),
            last_execute_time: maybe_parse_time(last_execute_time),
        })
        .unwrap()
    }
}

impl<'a> From<ExistingWorkspace<'a>> for Workspace {
    fn from(value: ExistingWorkspace) -> Self {
        let ExistingWorkspace {
            id,
            name,
            location,
            last_access_time,
        } = value;

        Workspace::new(WorkspaceParameters {
            id: parse_uuid(id),
            name: name.to_string(),
            location: location.map(ToString::to_string),
            last_access_time: maybe_parse_time(last_access_time),
        })
        .unwrap()
    }
}

impl<'a> From<ExpectedWorkspace<'a>> for Workspace {
    fn from(value: ExpectedWorkspace) -> Self {
        let ExpectedWorkspace {
            id,
            name,
            location,
            last_access_time,
        } = value;

        Workspace::new(WorkspaceParameters {
            id: parse_uuid(id),
            name: name.to_string(),
            location: location.map(ToString::to_string),
            last_access_time: maybe_parse_time(last_access_time),
        })
        .unwrap()
    }
}

impl<'a> From<ExpectedCommand<'a>> for Command {
    fn from(value: ExpectedCommand) -> Self {
        let ExpectedCommand {
            id,
            name,
            program,
            last_execute_time,
            workspace_id,
        } = value;

        Command::new(CommandParameters {
            id: parse_uuid(id),
            name: name.to_string(),
            program: program.to_string(),
            last_execute_time: maybe_parse_time(last_execute_time),
            workspace_id: parse_workspace_id(workspace_id),
        })
        .unwrap()
    }
}
