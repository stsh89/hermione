use crate::solutions::{workspace_fixture, InMemoryStorageProvider, WorkspaceFixtureParameters};
use hermione_nexus::{
    definitions::Workspace,
    operations::{UpdateWorkspaceOperation, UpdateWorkspaceParameters},
    Error, Result,
};
use uuid::Uuid;

struct UpdateWorkspaceOperationTestContext {
    storage: InMemoryStorageProvider,
    workspace: Workspace,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(UpdateWorkspaceOperationTestContext) -> Result<()>,
{
    let storage = InMemoryStorageProvider::new();
    let workspace = workspace_fixture(WorkspaceFixtureParameters {
        name: Some("Test workspace".to_string()),
        ..Default::default()
    })?;

    storage.insert_workspace(&workspace)?;

    test_fn(UpdateWorkspaceOperationTestContext { storage, workspace })
}

#[test]
fn it_updates_workspace() -> Result<()> {
    with_context(|ctx| {
        let UpdateWorkspaceOperationTestContext { storage, workspace } = ctx;

        assert_eq!(workspace.name(), "Test workspace");
        assert_eq!(workspace.location(), None);

        let workspace = UpdateWorkspaceOperation {
            find_workspace_provider: &storage,
            update_workspace_provider: &storage,
        }
        .execute(UpdateWorkspaceParameters {
            id: workspace.id(),
            name: "Spaceship".to_string(),
            location: Some("C:\\".to_string()),
        })?;

        assert_eq!(workspace.name(), "Spaceship");
        assert_eq!(workspace.location(), Some("C:\\"));

        Ok(())
    })
}

#[test]
fn it_returns_not_found_error() -> Result<()> {
    with_context(|ctx| {
        let UpdateWorkspaceOperationTestContext {
            storage,
            workspace: _,
        } = ctx;

        let result = UpdateWorkspaceOperation {
            find_workspace_provider: &storage,
            update_workspace_provider: &storage,
        }
        .execute(UpdateWorkspaceParameters {
            id: &Uuid::new_v4().into(),
            name: "Spaceship".to_string(),
            location: Some("C:\\".to_string()),
        });

        assert!(matches!(result, Err(Error::NotFound(_))));

        Ok(())
    })
}
