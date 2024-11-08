use chrono::Utc;
use hermione_drive::sqlite::{self, WorkspaceRecord};
use rusqlite::{Connection, Result};
use uuid::Uuid;

use crate::support::{workspace_record_fixture, WorkspaceRecordFixtureParameters};

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
        last_access_time: Utc::now().timestamp_nanos_opt(),
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

        let update = WorkspaceRecord {
            name: "Workspace 2".to_string(),
            ..workspace
        };

        let count = sqlite::update_workspace(&conn, update)?;

        assert_eq!(count, 1);

        let Some(updated) = sqlite::find_workspace(&conn, &workspace.id)? else {
            unreachable!("Workspace should be found")
        };

        assert_eq!(updated.name, "Workspace 2");

        Ok(())
    })
}

#[test]
fn it_updates_workspace_location() -> Result<()> {
    with_context(|ctx| {
        let UpdateWorkspaceTestContext { conn, workspace } = ctx;

        assert_eq!(workspace.location.as_deref(), Some("Location 1"));

        let update = WorkspaceRecord {
            location: Some("Location 2".to_string()),
            ..workspace
        };

        let count = sqlite::update_workspace(&conn, update)?;

        assert_eq!(count, 1);

        let Some(updated) = sqlite::find_workspace(&conn, &workspace.id)? else {
            unreachable!("Workspace should be found")
        };

        assert_eq!(updated.location.as_deref(), Some("Location 2"));

        Ok(())
    })
}

#[test]
fn it_updates_workspace_last_access_time() -> Result<()> {
    with_context(|ctx| {
        let UpdateWorkspaceTestContext { conn, workspace } = ctx;

        let update = WorkspaceRecord {
            last_access_time: workspace.last_access_time.map(|time| time + 1),
            ..workspace
        };

        let count = sqlite::update_workspace(&conn, update)?;

        assert_eq!(count, 1);

        let Some(updated) = sqlite::find_workspace(&conn, &workspace.id)? else {
            unreachable!("Workspace should be found")
        };

        assert_eq!(
            updated.last_access_time.unwrap(),
            workspace.last_access_time.unwrap() + 1
        );

        Ok(())
    })
}

#[test]
fn it_does_not_update_workspace() -> Result<()> {
    with_context(|ctx| {
        let UpdateWorkspaceTestContext { conn, workspace } = ctx;

        let update = WorkspaceRecord {
            id: Uuid::nil().into_bytes(),
            ..workspace
        };

        let count = sqlite::update_workspace(&conn, update)?;

        assert_eq!(count, 0);

        let Some(updated) = sqlite::find_workspace(&conn, &workspace.id)? else {
            unreachable!("Workspace should be found")
        };

        assert_eq!(updated.name, "Workspace 1");
        assert_eq!(updated.location.as_deref(), Some("Location 1"));
        assert_eq!(updated.last_access_time, workspace.last_access_time);

        Ok(())
    })
}
