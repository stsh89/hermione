use crate::storage::InMemoryStorageProvider;
use hermione_nexus::{CreateWorkspaceOperation, CreateWorkspaceParameters, Result};

#[test]
fn it_creates_workspace() -> Result<()> {
    let workspace = CreateWorkspaceOperation {
        operator: &InMemoryStorageProvider::new(),
    }
    .execute(CreateWorkspaceParameters {
        name: "Test workspace".to_string(),
        location: Some("/home/ironman".to_string()),
    })?;

    assert_eq!(workspace.name(), "Test workspace");
    assert_eq!(workspace.location(), Some("/home/ironman"));
    assert_eq!(workspace.last_access_time(), None);

    Ok(())
}
