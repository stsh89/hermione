use crate::support::{self, ExistingCommand, ExistingWorkspace, ExpectedCommand, InMemoryStorage};
use hermione_nexus::{
    definitions::Command,
    operations::{UpdateCommandOperation, UpdateCommandParameters},
    Error,
};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub struct ExistingStorageData<'a> {
    pub workspace: ExistingWorkspace<'a>,
    pub command: ExistingCommand<'a>,
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
            support::assert_command_new(operation_result.unwrap(), expected_command);
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

    support::insert_workspace_new(&background.storage, workspace);
    support::insert_command_new(&background.storage, command);
}
