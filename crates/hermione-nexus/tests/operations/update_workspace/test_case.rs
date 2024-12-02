use crate::support::{self, ExpectedWorkspace, InMemoryStorage, WorkspaceFixture};
use hermione_nexus::{
    definitions::Workspace,
    operations::{UpdateWorkspaceOperation, UpdateWorkspaceParameters},
    Error,
};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub enum OperationResult<'a> {
    Success {
        expected_workspace: ExpectedWorkspace<'a>,
    },
}

pub fn assert_operation_succeess(
    operation_result: Result<Workspace, Error>,
    expected: OperationResult,
) {
    match expected {
        OperationResult::Success { expected_workspace } => {
            assert!(operation_result.is_ok());
            support::assert_workspace(operation_result.unwrap(), expected_workspace);
        }
    }
}

pub fn execute_operation(
    background: &Background,
    parameters: UpdateWorkspaceParameters,
) -> Result<Workspace, Error> {
    let Background { storage } = background;

    UpdateWorkspaceOperation {
        find_workspace_provider: storage,
        update_workspace_provider: storage,
    }
    .execute(parameters)
}

pub fn setup(background: &Background, workspace: WorkspaceFixture) {
    support::insert_workspace(&background.storage, workspace);
}
