use crate::support::{self, ExistingWorkspace, ExpectedWorkspace, InMemoryStorage};
use hermione_nexus::{
    definitions::Workspace,
    operations::{ListWorkspacesOperation, ListWorkspacesParameters},
    Error,
};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub enum ExpectedOperationResult<'a> {
    Success {
        expected_workspaces: Vec<ExpectedWorkspace<'a>>,
    },
}

pub fn assert_operation_result(
    result: Result<Vec<Workspace>, Error>,
    expected: ExpectedOperationResult,
) {
    match expected {
        ExpectedOperationResult::Success {
            expected_workspaces,
        } => {
            assert!(result.is_ok());
            support::assert_workspaces(result.unwrap(), expected_workspaces)
        }
    }
}

pub fn execute_operation(
    backgound: &Background,
    parameters: ListWorkspacesParameters,
) -> Result<Vec<Workspace>, Error> {
    let Background { storage } = backgound;

    ListWorkspacesOperation { provider: storage }.execute(parameters)
}

pub fn setup(backgound: &Background, workspaces: Vec<ExistingWorkspace>) {
    let Background { storage } = backgound;

    support::insert_workspaces(storage, workspaces);
}
