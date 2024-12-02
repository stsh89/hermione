mod backup;
mod storage;
mod system;

pub use backup::*;
pub use storage::InMemoryStorage;
pub use system::MockSystem;

use chrono::{DateTime, NaiveDateTime, Utc};
use hermione_nexus::definitions::{
    Command, CommandId, CommandParameters, Workspace, WorkspaceId, WorkspaceParameters,
};
use uuid::Uuid;

pub struct CommandFixture<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub program: &'a str,
    pub last_execute_time: Option<&'a str>,
    pub workspace_id: &'a str,
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

pub struct WorkspaceFixture<'a> {
    pub id: &'a str,
    pub last_access_time: Option<&'a str>,
    pub location: Option<&'a str>,
    pub name: &'a str,
}

pub fn assert_clipboard_content(system: &MockSystem, expected: &str) {
    let content = system.clipboard.read().unwrap();

    assert_eq!(content.as_deref(), Some(expected));
}

pub fn assert_command(command: Command, expected: ExpectedCommand) {
    let expected = Command::from(expected);

    assert_eq!(command.id(), expected.id());
    assert_eq!(command.name(), expected.name());
    assert_eq!(command.program(), expected.program());
    assert_eq!(command.last_execute_time(), expected.last_execute_time(),);
}

pub fn assert_commands(commands: Vec<Command>, expected_commands: Vec<ExpectedCommand>) {
    assert_eq!(commands.len(), expected_commands.len());

    expected_commands
        .into_iter()
        .zip(commands)
        .for_each(|(expected, command)| assert_command(command, expected));
}

pub fn assert_file_system_location(system: &MockSystem, expected: &str) {
    let location = system.location.read().unwrap();

    assert_eq!(location.as_deref(), Some(expected));
}

pub fn assert_last_executed_program(system: &MockSystem, expected: &str) {
    let program = system.program.read().unwrap();

    assert_eq!(program.as_deref(), Some(expected));
}

pub fn assert_workspace(workspace: Workspace, expected: ExpectedWorkspace) {
    let expected = Workspace::from(expected);

    assert_eq!(workspace.id(), expected.id());
    assert_eq!(workspace.name(), expected.name());
    assert_eq!(workspace.location(), expected.location());
    assert_eq!(workspace.last_access_time(), expected.last_access_time(),);
}

pub fn assert_workspaces(workspaces: Vec<Workspace>, expected_workspaces: Vec<ExpectedWorkspace>) {
    assert_eq!(workspaces.len(), expected_workspaces.len());

    expected_workspaces
        .into_iter()
        .zip(workspaces)
        .for_each(|(expected, workspace)| assert_workspace(workspace, expected));
}

pub fn freeze_storage_time(storage: &InMemoryStorage, time: DateTime<Utc>) {
    *storage.now.write().unwrap() = Some(time);
}

pub fn get_command(storage: &InMemoryStorage, id: CommandId) -> Command {
    maybe_get_command(storage, id).unwrap_or_else(|| panic!("Command {} should exist", id))
}

pub fn get_workspace(storage: &InMemoryStorage, id: WorkspaceId) -> Workspace {
    maybe_get_workspace(storage, id).unwrap_or_else(|| panic!("Workspace {} should exist", id))
}

pub fn insert_command(storage: &InMemoryStorage, existing: CommandFixture) {
    let command = Command::from(existing);

    storage
        .commands
        .write()
        .unwrap()
        .insert(command.id(), command);
}

pub fn insert_commands(storage: &InMemoryStorage, commands: Vec<CommandFixture>) {
    commands
        .into_iter()
        .for_each(|command| insert_command(storage, command));
}

pub fn insert_workspace(storage: &InMemoryStorage, existing: WorkspaceFixture) {
    let workspace = Workspace::from(existing);

    storage
        .workspaces
        .write()
        .unwrap()
        .insert(workspace.id(), workspace);
}

pub fn insert_workspaces(storage: &InMemoryStorage, workspaces: Vec<WorkspaceFixture>) {
    workspaces
        .into_iter()
        .for_each(|workspace| insert_workspace(storage, workspace));
}

pub fn maybe_get_command(storage: &InMemoryStorage, id: CommandId) -> Option<Command> {
    storage.commands.read().unwrap().get(&id).cloned()
}

pub fn maybe_get_workspace(storage: &InMemoryStorage, id: WorkspaceId) -> Option<Workspace> {
    storage.workspaces.read().unwrap().get(&id).cloned()
}

pub fn maybe_parse_time(value: Option<&str>) -> Option<DateTime<Utc>> {
    value.map(parse_time)
}

pub fn parse_command_id(value: &str) -> CommandId {
    CommandId::parse_str(value).unwrap()
}

pub fn parse_time(value: &str) -> DateTime<Utc> {
    NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
        .unwrap()
        .and_utc()
}

pub fn parse_uuid(value: &str) -> Uuid {
    Uuid::parse_str(value).unwrap()
}

pub fn parse_workspace_id(value: &str) -> WorkspaceId {
    WorkspaceId::parse_str(value).unwrap()
}

impl<'a> From<CommandFixture<'a>> for Command {
    fn from(value: CommandFixture) -> Self {
        let CommandFixture {
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

impl<'a> From<WorkspaceFixture<'a>> for Workspace {
    fn from(value: WorkspaceFixture) -> Self {
        let WorkspaceFixture {
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
