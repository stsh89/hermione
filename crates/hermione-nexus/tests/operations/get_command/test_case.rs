use crate::support::{self, ExistingCommand, ExistingWorkspace, ExpectedCommand, InMemoryStorage};
use hermione_nexus::{definitions::Command, operations::GetCommandOperation, Result};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub struct ExistingStorageData<'a> {
    pub workspace: ExistingWorkspace<'a>,
    pub command: ExistingCommand<'a>,
}

pub enum ExpectedOperationResult<'a> {
    Success {
        expected_command: ExpectedCommand<'a>,
    },
}

pub fn assert_operation_result(result: Result<Command>, expected: ExpectedOperationResult) {
    match expected {
        ExpectedOperationResult::Success { expected_command } => {
            assert!(result.is_ok());
            support::assert_command_new(result.unwrap(), expected_command)
        }
    }
}

pub fn execute_operation(backgound: &Background, command_id: &str) -> Result<Command> {
    let Background { storage } = backgound;

    GetCommandOperation { provider: storage }.execute(support::parse_command_id(command_id))
}

pub fn setup(backgound: &Background, data: ExistingStorageData) {
    let Background { storage } = backgound;

    let ExistingStorageData { workspace, command } = data;

    support::insert_workspace_new(storage, workspace);
    support::insert_command_new(storage, command);
}
