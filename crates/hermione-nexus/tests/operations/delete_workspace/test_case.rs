use crate::support::{self, ExistingWorkspace, InMemoryStorage};
use hermione_nexus::{operations::DeleteWorkspaceOperation, Result};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub fn assert_operation_success(operation_result: Result<()>) {
    assert!(operation_result.is_ok());
}

pub fn assert_storage_does_not_contain_workspace(background: &Background, workspace_id: &str) {
    let workspace = support::maybe_get_workspace(
        &background.storage,
        support::parse_workspace_id(workspace_id),
    );

    assert!(workspace.is_none());
}

pub fn execute_operation(backgournd: &Background, workspace_id: &str) -> Result<()> {
    let Background { storage } = backgournd;

    DeleteWorkspaceOperation {
        find_workspace_provider: storage,
        delete_workspace_provider: storage,
    }
    .execute(support::parse_workspace_id(workspace_id))
}

pub fn setup(backgournd: &Background, workspace: ExistingWorkspace) {
    let Background { storage } = backgournd;

    support::insert_workspace_new(storage, workspace);
}
