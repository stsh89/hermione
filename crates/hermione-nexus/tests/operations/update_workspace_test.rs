use crate::{
    fixtures::{workspace_fixture, WorkspaceFixtureParameters},
    storage::InMemoryStorageProvider,
};
use hermione_nexus::{
    operations::{UpdateWorkspaceOperation, UpdateWorkspaceParameters},
    Error, Result,
};

#[test]
fn it_updates_workspace() -> Result<()> {
    let workspace = workspace_fixture(WorkspaceFixtureParameters {
        name: Some("Test workspace".to_string()),
        location: Some("/home/ironman".to_string()),
        ..Default::default()
    })?;

    let storage_provider = InMemoryStorageProvider::new();
    storage_provider.insert_workspace(&workspace)?;

    let workspace = UpdateWorkspaceOperation {
        find_provider: &storage_provider,
        update_provider: &storage_provider,
    }
    .execute(UpdateWorkspaceParameters {
        id: workspace.id(),
        name: "Spaceship".to_string(),
        location: Some("C:\\".to_string()),
    })?;

    assert_eq!(workspace.name(), "Spaceship");
    assert_eq!(workspace.location(), Some("C:\\"));
    assert_eq!(workspace.last_access_time(), None);

    Ok(())
}

#[test]
fn it_returns_workspace_not_found_error() -> Result<()> {
    let workspace = workspace_fixture(Default::default())?;
    let storage_provider = InMemoryStorageProvider::new();

    let result = UpdateWorkspaceOperation {
        find_provider: &storage_provider,
        update_provider: &storage_provider,
    }
    .execute(UpdateWorkspaceParameters {
        id: workspace.id(),
        name: "Spaceship".to_string(),
        location: Some("C:\\".to_string()),
    });

    match result {
        Err(error) => assert!(matches!(error, Error::NotFound(_))),
        Ok(_) => unreachable!(),
    };

    Ok(())
}
