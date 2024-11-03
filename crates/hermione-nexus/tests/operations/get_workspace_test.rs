use crate::solutions::{workspace_fixture, InMemoryStorageProvider};
use hermione_nexus::{definitions::Workspace, operations::GetWorkspaceOperation, Error, Result};
use uuid::Uuid;

struct GetWorkspaceOperationTestContext {
    storage: InMemoryStorageProvider,
    workspace: Workspace,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(GetWorkspaceOperationTestContext) -> Result<()>,
{
    let storage = InMemoryStorageProvider::new();
    let workspace = workspace_fixture(Default::default())?;

    storage.insert_workspace(&workspace)?;

    test_fn(GetWorkspaceOperationTestContext { storage, workspace })
}

#[test]
fn it_returns_workspace() -> Result<()> {
    with_context(|ctx| {
        let GetWorkspaceOperationTestContext { storage, workspace } = ctx;

        let found = GetWorkspaceOperation { provider: &storage }.execute(workspace.id())?;

        assert_eq!(found.name(), workspace.name());

        Ok(())
    })
}

#[test]
fn it_returns_not_found_error() -> Result<()> {
    with_context(|ctx| {
        let GetWorkspaceOperationTestContext {
            storage,
            workspace: _,
        } = ctx;

        let result = GetWorkspaceOperation { provider: &storage }.execute(&Uuid::nil().into());

        assert!(matches!(result, Err(Error::NotFound(_))));

        Ok(())
    })
}
