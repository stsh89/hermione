use crate::support::{self, ExpectedWorkspace, InMemoryStorage};
use hermione_nexus::{
    definitions::Workspace,
    operations::{CreateWorkspaceOperation, CreateWorkspaceParameters},
    Result,
};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub fn assert_returned_workspace(workspace: Workspace, expected: ExpectedWorkspace) {
    support::assert_workspace(workspace, expected)
}

pub fn assert_storage_contains_workspace(background: &Background, expected: ExpectedWorkspace) {
    let workspace = support::get_workspace(&background.storage, expected.id());

    support::assert_workspace(workspace, expected);
}

pub fn execute_operation(
    background: &Background,
    parameters: CreateWorkspaceParameters,
) -> Result<Workspace> {
    let Background { storage } = background;

    CreateWorkspaceOperation {
        storage_provider: storage,
    }
    .execute(parameters)
}
