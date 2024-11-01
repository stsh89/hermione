use crate::{
    fixtures::{workspace_fixture, WorkspaceFixtureParameters},
    storage::InMemoryStorageProvider,
};
use chrono::{TimeZone, Utc};
use hermione_nexus::{Error, ListWorkspacesOperation, ListWorkspacesParameters, Result};
use uuid::Uuid;

#[test]
fn it_lists_workspaces() -> Result<()> {
    let storage_provider = InMemoryStorageProvider::new();

    let workspace = workspace_fixture(WorkspaceFixtureParameters {
        name: Some("Test workspace".to_string()),
        ..Default::default()
    })?;

    storage_provider.insert_workspace(&workspace)?;

    let workspaces = ListWorkspacesOperation {
        operator: &storage_provider,
    }
    .execute(ListWorkspacesParameters {
        name_contains: None,
        page_number: 1,
        page_size: 1,
    })?;

    assert_eq!(
        workspaces
            .into_iter()
            .map(|v| **v.id())
            .collect::<Vec<Uuid>>(),
        vec![**workspace.id()]
    );

    Ok(())
}

#[test]
fn it_paginates() -> Result<()> {
    let storage_provider = InMemoryStorageProvider::new();

    let workspace1 = workspace_fixture(WorkspaceFixtureParameters {
        name: Some("Test workspace 1".to_string()),
        ..Default::default()
    })?;

    storage_provider.insert_workspace(&workspace1)?;

    let workspace2 = workspace_fixture(WorkspaceFixtureParameters {
        name: Some("Test workspace 2".to_string()),
        ..Default::default()
    })?;

    storage_provider.insert_workspace(&workspace2)?;

    let workspaces = ListWorkspacesOperation {
        operator: &storage_provider,
    }
    .execute(ListWorkspacesParameters {
        name_contains: None,
        page_number: 2,
        page_size: 1,
    })?;

    assert_eq!(
        workspaces
            .into_iter()
            .map(|v| **v.id())
            .collect::<Vec<Uuid>>(),
        vec![**workspace2.id()]
    );

    Ok(())
}

#[test]
fn it_filters_by_name() -> Result<()> {
    let storage_provider = InMemoryStorageProvider::new();

    let workspace1 = workspace_fixture(WorkspaceFixtureParameters {
        name: Some("Test workspace 1".to_string()),
        ..Default::default()
    })?;

    storage_provider.insert_workspace(&workspace1)?;

    let workspace2 = workspace_fixture(WorkspaceFixtureParameters {
        name: Some("Test workspace 2".to_string()),
        ..Default::default()
    })?;

    storage_provider.insert_workspace(&workspace2)?;

    let workspace3 = workspace_fixture(WorkspaceFixtureParameters {
        name: Some("Spaceship".to_string()),
        ..Default::default()
    })?;

    storage_provider.insert_workspace(&workspace3)?;

    let workspaces = ListWorkspacesOperation {
        operator: &storage_provider,
    }
    .execute(ListWorkspacesParameters {
        name_contains: Some("Test"),
        page_number: 1,
        page_size: 10,
    })?;

    assert_eq!(
        workspaces
            .into_iter()
            .map(|v| **v.id())
            .collect::<Vec<Uuid>>(),
        vec![**workspace1.id(), **workspace2.id()]
    );

    Ok(())
}

#[test]
fn it_validates_page_number() -> Result<()> {
    let storage_provider = InMemoryStorageProvider::new();

    let result = ListWorkspacesOperation {
        operator: &storage_provider,
    }
    .execute(ListWorkspacesParameters {
        name_contains: None,
        page_number: 0,
        page_size: 10,
    });

    match result {
        Err(Error::InvalidArgument(description)) => {
            assert_eq!(description, "Page number must be greater than 0");
        }
        _ => unreachable!(),
    }

    Ok(())
}

#[test]
fn it_validates_page_size() -> Result<()> {
    let storage_provider = InMemoryStorageProvider::new();

    let result = ListWorkspacesOperation {
        operator: &storage_provider,
    }
    .execute(ListWorkspacesParameters {
        name_contains: None,
        page_number: 1,
        page_size: 0,
    });

    match result {
        Err(Error::InvalidArgument(description)) => {
            assert_eq!(description, "Page size must be greater than 0");
        }
        _ => unreachable!(),
    }

    Ok(())
}

#[test]
fn it_sorts_workspaces_by_last_access_time_and_name() -> Result<()> {
    let storage_provider = InMemoryStorageProvider::new();

    let workspace1 = workspace_fixture(WorkspaceFixtureParameters {
        name: Some("Test workspace 1".to_string()),
        ..Default::default()
    })?;

    storage_provider.insert_workspace(&workspace1)?;

    let workspace2 = workspace_fixture(WorkspaceFixtureParameters {
        name: Some("Test workspace 2".to_string()),
        last_access_time: Some(Utc.with_ymd_and_hms(2024, 10, 31, 10, 0, 0).unwrap()),
        ..Default::default()
    })?;

    storage_provider.insert_workspace(&workspace2)?;

    let workspace3 = workspace_fixture(WorkspaceFixtureParameters {
        name: Some("Spaceship".to_string()),
        ..Default::default()
    })?;

    storage_provider.insert_workspace(&workspace3)?;

    let workspaces = ListWorkspacesOperation {
        operator: &storage_provider,
    }
    .execute(ListWorkspacesParameters {
        name_contains: None,
        page_number: 1,
        page_size: 10,
    })?;

    assert_eq!(
        workspaces.iter().map(|v| v.name()).collect::<Vec<_>>(),
        vec![workspace2.name(), workspace3.name(), workspace1.name()]
    );

    Ok(())
}
