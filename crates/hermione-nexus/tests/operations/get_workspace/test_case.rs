use crate::support::{self, ExistingWorkspace, ExpectedWorkspace, InMemoryStorage};
use hermione_nexus::{definitions::Workspace, operations::GetWorkspaceOperation, Result};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub enum ExpectedOperationResult<'a> {
    Success {
        expected_workspace: ExpectedWorkspace<'a>,
    },
}

pub fn assert_operation_result(result: Result<Workspace>, expected: ExpectedOperationResult) {
    match expected {
        ExpectedOperationResult::Success { expected_workspace } => {
            assert!(result.is_ok());
            support::assert_workspace_new(result.unwrap(), expected_workspace)
        }
    }
}

pub fn execute_operation(backgound: &Background, workspace_id: &str) -> Result<Workspace> {
    let Background { storage } = backgound;

    GetWorkspaceOperation { provider: storage }.execute(support::parse_workspace_id(workspace_id))
}

pub fn setup(backgound: &Background, workspace: ExistingWorkspace) {
    let Background { storage } = backgound;

    support::insert_workspace_new(storage, workspace);
}
