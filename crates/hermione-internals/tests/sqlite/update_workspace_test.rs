use crate::support::{self, workspace_record_fixture, WorkspaceRecordFixtureParameters};
use hermione_internals::sqlite::{
    self, OptionalValue, UpdateWorkspaceQueryOptions, WorkspaceRecord,
};
use rusqlite::{Connection, Result};

struct UpdateWorkspaceTestContext {
    conn: Connection,
    workspace: WorkspaceRecord,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(UpdateWorkspaceTestContext) -> Result<()>,
{
    let conn = Connection::open_in_memory()?;
    sqlite::create_workspaces_table_if_not_exists(&conn)?;

    let workspace = workspace_record_fixture(WorkspaceRecordFixtureParameters {
        name: Some("Workspace 1".to_string()),
        location: Some("Location 1".to_string()),
        last_access_time: Some(10),
        ..Default::default()
    });

    sqlite::insert_workspace(&conn, workspace.clone())?;

    test_fn(UpdateWorkspaceTestContext { conn, workspace })
}

#[test]
fn it_updates_workspace_name() -> Result<()> {
    with_context(|ctx| {
        let UpdateWorkspaceTestContext { conn, workspace } = ctx;

        assert_eq!(workspace.name, "Workspace 1");

        let count = sqlite::update_workspace(
            &conn,
            UpdateWorkspaceQueryOptions {
                id: workspace.id,
                last_access_time: None,
                location: None,
                name: Some("Spaceship".to_string()),
            },
        )?;

        assert_eq!(count, 1);

        let workspace = support::query_workspace(&conn, &workspace.id)?;

        assert_eq!(workspace.name, "Spaceship");
        assert_eq!(workspace.location.as_deref(), Some("Location 1"));
        assert_eq!(workspace.last_access_time, Some(10));

        Ok(())
    })
}

#[test]
fn it_updates_workspace_location() -> Result<()> {
    with_context(|ctx| {
        let UpdateWorkspaceTestContext { conn, workspace } = ctx;

        assert_eq!(workspace.location.as_deref(), Some("Location 1"));

        let count = sqlite::update_workspace(
            &conn,
            UpdateWorkspaceQueryOptions {
                id: workspace.id,
                last_access_time: None,
                location: Some(OptionalValue::Value("/home/ironman".to_string())),
                name: None,
            },
        )?;

        assert_eq!(count, 1);

        let workspace = support::query_workspace(&conn, &workspace.id)?;

        assert_eq!(workspace.name, "Workspace 1");
        assert_eq!(workspace.location.as_deref(), Some("/home/ironman"));
        assert_eq!(workspace.last_access_time, Some(10));

        Ok(())
    })
}

#[test]
fn it_updates_workspace_location_with_none() -> Result<()> {
    with_context(|ctx| {
        let UpdateWorkspaceTestContext { conn, workspace } = ctx;

        assert_eq!(workspace.location.as_deref(), Some("Location 1"));

        let count = sqlite::update_workspace(
            &conn,
            UpdateWorkspaceQueryOptions {
                id: workspace.id,
                last_access_time: None,
                location: Some(OptionalValue::Null),
                name: None,
            },
        )?;

        assert_eq!(count, 1);

        let workspace = support::query_workspace(&conn, &workspace.id)?;

        assert_eq!(workspace.name, "Workspace 1");
        assert_eq!(workspace.location, None);
        assert_eq!(workspace.last_access_time, Some(10));

        Ok(())
    })
}

#[test]
fn it_updates_last_access_time() -> Result<()> {
    with_context(|ctx| {
        let UpdateWorkspaceTestContext { conn, workspace } = ctx;

        assert_eq!(workspace.last_access_time, Some(10));

        let count = sqlite::update_workspace(
            &conn,
            UpdateWorkspaceQueryOptions {
                id: workspace.id,
                last_access_time: Some(OptionalValue::Value(20)),
                location: None,
                name: None,
            },
        )?;

        assert_eq!(count, 1);

        let workspace = support::query_workspace(&conn, &workspace.id)?;

        assert_eq!(workspace.name, "Workspace 1");
        assert_eq!(workspace.location.as_deref(), Some("Location 1"));
        assert_eq!(workspace.last_access_time, Some(20));

        Ok(())
    })
}

#[test]
fn it_updates_last_access_time_with_none() -> Result<()> {
    with_context(|ctx| {
        let UpdateWorkspaceTestContext { conn, workspace } = ctx;

        assert_eq!(workspace.last_access_time, Some(10));

        let count = sqlite::update_workspace(
            &conn,
            UpdateWorkspaceQueryOptions {
                id: workspace.id,
                last_access_time: Some(OptionalValue::Null),
                location: None,
                name: None,
            },
        )?;

        assert_eq!(count, 1);

        let workspace = support::query_workspace(&conn, &workspace.id)?;

        assert_eq!(workspace.name, "Workspace 1");
        assert_eq!(workspace.location.as_deref(), Some("Location 1"));
        assert_eq!(workspace.last_access_time, None);

        Ok(())
    })
}

#[test]
fn it_does_not_update_workspace_when_nothing_changed() -> Result<()> {
    with_context(|ctx| {
        let UpdateWorkspaceTestContext { conn, workspace } = ctx;

        assert_eq!(workspace.name, "Workspace 1");
        assert_eq!(workspace.location.as_deref(), Some("Location 1"));
        assert_eq!(workspace.last_access_time, Some(10));

        let count = sqlite::update_workspace(
            &conn,
            UpdateWorkspaceQueryOptions {
                id: workspace.id,
                last_access_time: None,
                location: None,
                name: None,
            },
        )?;

        assert_eq!(count, 0);

        let workspace = support::query_workspace(&conn, &workspace.id)?;

        assert_eq!(workspace.name, "Workspace 1");
        assert_eq!(workspace.location.as_deref(), Some("Location 1"));
        assert_eq!(workspace.last_access_time, Some(10));

        Ok(())
    })
}
