use crate::support::{self, CommandFixture, ExpectedCommand, InMemoryStorage, WorkspaceFixture};
use hermione_nexus::{
    definitions::Command,
    operations::{UpdateCommandOperation, UpdateCommandParameters},
    Error,
};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub struct ExistingStorageData<'a> {
    pub workspace: WorkspaceFixture<'a>,
    pub command: CommandFixture<'a>,
}

pub enum OperationResult<'a> {
    Success {
        expected_command: ExpectedCommand<'a>,
    },
}

pub fn assert_operation_succeess(
    operation_result: Result<Command, Error>,
    expected: OperationResult,
) {
    match expected {
        OperationResult::Success { expected_command } => {
            assert!(operation_result.is_ok());
            support::assert_command(operation_result.unwrap(), expected_command);
        }
    }
}

pub fn execute_operation(
    background: &Background,
    parameters: UpdateCommandParameters,
) -> Result<Command, Error> {
    let Background { storage } = background;

    UpdateCommandOperation {
        find_command_provider: storage,
        update_command_provider: storage,
    }
    .execute(parameters)
}

pub fn setup(background: &Background, data: ExistingStorageData) {
    let ExistingStorageData { command, workspace } = data;

    support::insert_workspace(&background.storage, workspace);
    support::insert_command(&background.storage, command);
}
