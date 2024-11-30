use crate::support::{self, InMemoryStorage};
use hermione_nexus::{
    definitions::{Workspace, WorkspaceId, WorkspaceParameters},
    operations::DeleteWorkspaceOperation,
    Result,
};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub struct ExistingWorkspace<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

pub fn assert_operation_success(operation_result: Result<()>) {
    assert!(operation_result.is_ok());
}

pub fn assert_storage_does_not_contain_workspace(background: &Background, workspace_id: &str) {
    let workspace_id = WorkspaceId::parse_str(workspace_id).unwrap();
    let workspaces = background.storage.workspaces.read().unwrap();
    let workspace = workspaces.get(&workspace_id);

    assert!(workspace.is_none());
}

pub fn execute_operation(backgournd: &Background, workspace_id: &str) -> Result<()> {
    let Background { storage } = backgournd;

    let workspace_id = WorkspaceId::parse_str(workspace_id).unwrap();

    DeleteWorkspaceOperation {
        find_workspace_provider: storage,
        delete_workspace_provider: storage,
    }
    .execute(workspace_id)
}

pub fn setup(backgournd: &Background, workspace: ExistingWorkspace) {
    let Background { storage } = backgournd;

    let ExistingWorkspace { id, name } = workspace;

    let id = id.parse().unwrap();

    let workspace = Workspace::new(WorkspaceParameters {
        id,
        name: name.to_string(),
        location: None,
        last_access_time: None,
    })
    .unwrap();

    support::insert_raw_workspace(storage, workspace);
}
