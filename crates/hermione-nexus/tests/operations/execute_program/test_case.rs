use crate::support::{self, InMemoryStorage, MockSystem, WorkspaceFixture};
use hermione_nexus::{
    operations::{ExecuteProgramOperation, ExecuteProgramParameters},
    Error,
};

pub struct Background {
    pub storage: InMemoryStorage,
    pub system: MockSystem,
}

pub enum ExpectedOperationResult {
    Success,
}

pub fn assert_executed_system_program(backgound: &Background, expected: &str) {
    let Background { system, .. } = backgound;

    support::assert_last_executed_program(system, expected);
}

pub fn assert_operation_result(result: Result<(), Error>, expected: ExpectedOperationResult) {
    match expected {
        ExpectedOperationResult::Success => assert!(result.is_ok()),
    }
}

pub fn execute_operation(
    backgound: &Background,
    parameters: ExecuteProgramParameters,
) -> Result<(), Error> {
    let Background { storage, system } = backgound;

    ExecuteProgramOperation {
        system,
        find_workspace: storage,
    }
    .execute(parameters)
}

pub fn setup(backgound: &Background, workspace: WorkspaceFixture) {
    let Background { storage, system: _ } = backgound;

    support::insert_workspace(storage, workspace);
}
