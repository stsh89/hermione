use crate::support::{self, CommandFixture, ExpectedCommand, InMemoryStorage, WorkspaceFixture};
use hermione_nexus::{definitions::Command, operations::GetCommandOperation, Result};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub struct ExistingStorageData<'a> {
    pub workspace: WorkspaceFixture<'a>,
    pub command: CommandFixture<'a>,
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
            support::assert_command(result.unwrap(), expected_command)
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

    support::insert_workspace(storage, workspace);
    support::insert_command(storage, command);
}
