use crate::support::{
    self, ExistingCommand, ExistingWorkspace, ExpectedCommand, ExpectedWorkspace, InMemoryStorage,
    MockSystem,
};
use hermione_nexus::{operations::ExecuteCommandOperation, Error};

pub struct Background {
    pub storage: InMemoryStorage,
    pub system: MockSystem,
}

pub struct BackgroundContext<'a> {
    pub workspace: ExistingWorkspace<'a>,
    pub command: ExistingCommand<'a>,
    pub time_freeze: &'a str,
}

pub enum ExpectedOperationResult {
    Success,
}

pub struct ExpectedStorageState<'a> {
    pub expected_command: ExpectedCommand<'a>,
    pub expected_workspace: ExpectedWorkspace<'a>,
}

pub struct ExpectedSystemChanges<'a> {
    pub last_executed_program: &'a str,
    pub last_visited_location: &'a str,
}

pub fn assert_storage_changes(backgound: &Background, expected: ExpectedStorageState) {
    let Background { storage, system: _ } = backgound;
    let ExpectedStorageState {
        expected_command,
        expected_workspace,
    } = expected;

    let command = support::get_command(storage, expected_command.id());
    let workspace = support::get_workspace(storage, expected_workspace.id());

    support::assert_command_new(command, expected_command);
    support::assert_workspace_new(workspace, expected_workspace);
}

pub fn assert_system_changes(backgound: &Background, expected: ExpectedSystemChanges) {
    let Background { system, .. } = backgound;

    let ExpectedSystemChanges {
        last_executed_program,
        last_visited_location,
    } = expected;

    support::assert_last_executed_program(system, last_executed_program);
    support::assert_system_location(system, last_visited_location);
}

pub fn assert_operation_result(result: Result<(), Error>, expected: ExpectedOperationResult) {
    match expected {
        ExpectedOperationResult::Success => assert!(result.is_ok()),
    }
}

pub fn execute_operation(backgound: &Background, command_id: &str) -> Result<(), Error> {
    let Background { storage, system } = backgound;

    ExecuteCommandOperation {
        find_command_provider: storage,
        find_workspace_provider: storage,
        system_provider: system,
        track_command_provider: storage,
        track_workspace_provider: storage,
    }
    .execute(support::parse_command_id(command_id))
}

pub fn setup(backgound: &Background, context: BackgroundContext) {
    let Background { storage, system: _ } = backgound;

    let BackgroundContext {
        workspace,
        command,
        time_freeze,
    } = context;

    support::insert_workspace_new(storage, workspace);
    support::insert_command_new(storage, command);
    support::freeze_storage_time(storage, support::parse_time(time_freeze));
}
