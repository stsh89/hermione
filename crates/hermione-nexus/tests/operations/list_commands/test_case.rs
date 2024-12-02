use crate::support::{self, CommandFixture, ExpectedCommand, InMemoryStorage, WorkspaceFixture};
use hermione_nexus::{
    definitions::Command,
    operations::{ListCommandsOperation, ListCommandsParameters},
    Error,
};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub struct BackgroundContext<'a> {
    pub workspace: WorkspaceFixture<'a>,
    pub commands: Vec<CommandFixture<'a>>,
}

pub enum ExpectedOperationResult<'a> {
    Success {
        expected_commands: Vec<ExpectedCommand<'a>>,
    },
}

pub fn assert_operation_result(
    result: Result<Vec<Command>, Error>,
    expected: ExpectedOperationResult,
) {
    match expected {
        ExpectedOperationResult::Success { expected_commands } => {
            assert!(result.is_ok());
            support::assert_commands(result.unwrap(), expected_commands)
        }
    }
}

pub fn execute_operation(
    backgound: &Background,
    parameters: ListCommandsParameters,
) -> Result<Vec<Command>, Error> {
    let Background { storage } = backgound;

    ListCommandsOperation { provider: storage }.execute(parameters)
}

pub fn setup(backgound: &Background, context: BackgroundContext) {
    let Background { storage } = backgound;

    let BackgroundContext {
        workspace,
        commands,
    } = context;

    support::insert_workspace(storage, workspace);
    support::insert_commands(storage, commands);
}
