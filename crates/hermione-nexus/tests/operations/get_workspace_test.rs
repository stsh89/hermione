use crate::{fixtures, storage::InMemoryStorageProvider};
use hermione_nexus::{operations::GetWorkspaceOperation, Error, Result};
use uuid::Uuid;

#[test]
fn it_returns_workspace() -> Result<()> {
    let workspace = fixtures::workspace_fixture(Default::default())?;

    let storage_provider = InMemoryStorageProvider::new();
    storage_provider.insert_workspace(&workspace)?;

    let found = GetWorkspaceOperation {
        provider: &storage_provider,
    }
    .execute(workspace.id())?;

    assert_eq!(**found.id(), **workspace.id());

    Ok(())
}

#[test]
fn it_returns_workspace_not_found_error() -> Result<()> {
    let storage_provider = InMemoryStorageProvider::new();
    let id = Uuid::new_v4();

    let result = GetWorkspaceOperation {
        provider: &storage_provider,
    }
    .execute(&id.into());

    match result {
        Err(Error::NotFound(description)) => {
            assert_eq!(description, format!("Workspace with ID: {}", id));
        }
        _ => unreachable!(),
    };

    Ok(())
}
