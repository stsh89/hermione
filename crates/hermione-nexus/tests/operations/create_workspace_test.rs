use crate::storage::InMemoryStorageProvider;
use hermione_nexus::{
    operations::{CreateWorkspaceOperation, CreateWorkspaceParameters},
    Result,
};

struct CreateWorkspaceOperationTestContext {
    storage: InMemoryStorageProvider,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(CreateWorkspaceOperationTestContext) -> Result<()>,
{
    let storage = InMemoryStorageProvider::new();

    test_fn(CreateWorkspaceOperationTestContext { storage })
}

#[test]
fn it_creates_workspace() -> Result<()> {
    with_context(|ctx| {
        let CreateWorkspaceOperationTestContext { storage } = ctx;

        assert_eq!(storage.workspaces_count()?, 0);

        let workspace =
            CreateWorkspaceOperation { provider: &storage }.execute(CreateWorkspaceParameters {
                name: "Test workspace".to_string(),
                location: Some("/home/ironman".to_string()),
            })?;

        assert_eq!(storage.workspaces_count()?, 1);
        assert_eq!(workspace.name(), "Test workspace");
        assert_eq!(workspace.location(), Some("/home/ironman"));
        assert_eq!(workspace.last_access_time(), None);

        Ok(())
    })
}
