use crate::support::{
    page_number, page_size, workspace_fixture, InMemoryStorage, WorkspaceFixtureParameters,
};
use chrono::Utc;
use hermione_nexus::{
    operations::{ListWorkspacesOperation, ListWorkspacesParameters},
    Result,
};

struct ListWorkspacesOperationTestContext {
    storage: InMemoryStorage,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(ListWorkspacesOperationTestContext) -> Result<()>,
{
    let storage = InMemoryStorage::default();

    for workspace_number in 1..=8 {
        let workspace = workspace_fixture(WorkspaceFixtureParameters {
            name: Some(format!("Test workspace {workspace_number}")),
            ..Default::default()
        })?;

        storage.insert_workspace(&workspace)?;
    }

    test_fn(ListWorkspacesOperationTestContext { storage })
}

#[test]
fn it_paginates() -> Result<()> {
    with_context(|ctx| {
        let ListWorkspacesOperationTestContext { storage } = ctx;

        let workspaces =
            ListWorkspacesOperation { provider: &storage }.execute(ListWorkspacesParameters {
                name_contains: None,
                page_number: page_number(2)?,
                page_size: page_size(3)?,
            })?;

        assert_eq!(
            workspaces.iter().map(|w| w.name()).collect::<Vec<_>>(),
            vec!["Test workspace 4", "Test workspace 5", "Test workspace 6",]
        );

        Ok(())
    })
}

#[test]
fn it_filters_by_name() -> Result<()> {
    with_context(|ctx| {
        let ListWorkspacesOperationTestContext { storage } = ctx;

        let workspaces =
            ListWorkspacesOperation { provider: &storage }.execute(ListWorkspacesParameters {
                name_contains: Some("7"),
                page_number: page_number(1)?,
                page_size: page_size(10)?,
            })?;

        assert_eq!(
            workspaces.iter().map(|w| w.name()).collect::<Vec<_>>(),
            vec!["Test workspace 7",]
        );

        Ok(())
    })
}

#[test]
fn it_sorts_workspaces_by_last_access_time_and_name() -> Result<()> {
    with_context(|ctx| {
        let ListWorkspacesOperationTestContext { storage } = ctx;

        let workspace = workspace_fixture(WorkspaceFixtureParameters {
            name: Some("Test workspace 9".to_string()),
            last_access_time: Some(Utc::now()),
            ..Default::default()
        })?;

        storage.insert_workspace(&workspace)?;

        let workspaces =
            ListWorkspacesOperation { provider: &storage }.execute(ListWorkspacesParameters {
                name_contains: None,
                page_number: page_number(1)?,
                page_size: page_size(3)?,
            })?;

        assert_eq!(
            workspaces.iter().map(|w| w.name()).collect::<Vec<_>>(),
            vec!["Test workspace 9", "Test workspace 1", "Test workspace 2",]
        );

        Ok(())
    })
}
