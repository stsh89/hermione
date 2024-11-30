use hermione_nexus::{
    definitions::{Workspace, WorkspaceId, WorkspaceParameters},
    operations::GetWorkspaceOperation,
    Result,
};
use uuid::Uuid;

use crate::support::{self, InMemoryStorage};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub struct ExistingWorkspace<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub location: Option<&'a str>,
    pub last_access_time: Option<&'a str>,
}

pub enum ExpectedOperationResult<'a> {
    Success {
        expected_workspace: ExpectedWorkspace<'a>,
    },
}

pub struct ExpectedWorkspace<'a> {
    pub id: &'a str,
    pub last_access_time: Option<&'a str>,
    pub location: Option<&'a str>,
    pub name: &'a str,
}

pub fn assert_operation_result(result: Result<Workspace>, expected: ExpectedOperationResult) {
    match expected {
        ExpectedOperationResult::Success { expected_workspace } => {
            assert!(result.is_ok());
            assert_workspace(result.unwrap(), expected_workspace)
        }
    }
}

pub fn assert_workspace(workspace: Workspace, expected: ExpectedWorkspace) {
    let ExpectedWorkspace {
        id,
        last_access_time,
        location,
        name,
    } = expected;

    let id = WorkspaceId::parse_str(id).unwrap();
    let last_access_time = support::maybe_parse_time(last_access_time);

    assert_eq!(workspace.id(), id);
    assert_eq!(workspace.name(), name);
    assert_eq!(workspace.location(), location);
    assert_eq!(workspace.last_access_time(), last_access_time.as_ref());
}

pub fn execute_operation(backgound: &Background, workspace_id: &str) -> Result<Workspace> {
    let Background { storage } = backgound;

    let workspace_id = WorkspaceId::parse_str(workspace_id).unwrap();

    GetWorkspaceOperation { provider: storage }.execute(workspace_id)
}

pub fn setup(backgound: &Background, workspace: ExistingWorkspace) {
    let Background { storage } = backgound;

    let ExistingWorkspace {
        id,
        name,
        location,
        last_access_time,
    } = workspace;

    let id = Uuid::parse_str(id).unwrap();
    let last_access_time = support::maybe_parse_time(last_access_time);

    let workspace = Workspace::new(WorkspaceParameters {
        id,
        name: name.to_string(),
        location: location.map(ToString::to_string),
        last_access_time,
    })
    .unwrap();

    support::insert_raw_workspace(storage, workspace);
}
