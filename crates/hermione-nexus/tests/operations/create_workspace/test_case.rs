use crate::support::InMemoryStorage;
use chrono::{DateTime, Utc};
use hermione_nexus::{
    definitions::Workspace,
    operations::{CreateWorkspaceOperation, CreateWorkspaceParameters},
    Result,
};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub enum ExpectedOperationResult<'a> {
    Success {
        expected_workspace: ExpectedWorkspace<'a>,
    },
}

pub struct ExpectedWorkspace<'a> {
    pub name: &'a str,
    pub location: Option<&'a str>,
    pub last_access_time: Option<&'a DateTime<Utc>>,
}

pub struct ExpectedStoredWorkspace<'a> {
    pub name: &'a str,
    pub location: Option<&'a str>,
    pub last_access_time: Option<&'a DateTime<Utc>>,
}

pub fn assert_operation_result(result: Result<Workspace>, expected: ExpectedOperationResult) {
    match expected {
        ExpectedOperationResult::Success { expected_workspace } => {
            assert!(result.is_ok());
            assert_workspace(result.unwrap(), expected_workspace)
        }
    }
}

fn assert_workspace(workspace: Workspace, expected: ExpectedWorkspace) {
    let ExpectedWorkspace {
        name,
        location,
        last_access_time,
    } = expected;

    assert_eq!(workspace.name(), name);
    assert_eq!(workspace.location(), location);
    assert_eq!(workspace.last_access_time(), last_access_time);
}

pub fn assert_storage_contains_workspace(
    storage: &InMemoryStorage,
    expected: ExpectedStoredWorkspace,
) {
    let workspaces = storage.workspaces.read().unwrap();

    assert_eq!(workspaces.len(), 1);

    let (_, workspace) = workspaces.iter().next().unwrap();

    let ExpectedStoredWorkspace {
        name,
        location,
        last_access_time,
    } = expected;

    assert_eq!(workspace.name(), name);
    assert_eq!(workspace.location(), location);
    assert_eq!(workspace.last_access_time(), last_access_time);
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
