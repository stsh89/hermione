use crate::solutions::{workspace_record_fixture, WorkspaceRecordFixtureParameters};
use hermione_drive::sqlite::{self, WorkspaceRecord};
use rusqlite::{Connection, Result};
use uuid::Uuid;

struct FindWorkspaceTestContest {
    conn: Connection,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(FindWorkspaceTestContest) -> Result<()>,
{
    let conn = Connection::open_in_memory()?;
    sqlite::create_workspaces_table_if_not_exists(&conn)?;

    test_fn(FindWorkspaceTestContest { conn })
}

#[test]
fn it_returns_workspace() -> Result<()> {
    with_context(|ctx| {
        let FindWorkspaceTestContest { conn } = ctx;

        let record = workspace_record_fixture(WorkspaceRecordFixtureParameters {
            name: Some("Workspace 1".to_string()),
            location: Some("Location 1".to_string()),
            last_access_time: Some(1),
            ..Default::default()
        });
        sqlite::insert_workspace(&conn, record.clone())?;

        let Some(WorkspaceRecord {
            id,
            last_access_time,
            location,
            name,
        }) = sqlite::find_workspace(&conn, &record.id)?
        else {
            unreachable!("Expected record to be found")
        };

        assert_eq!(id, record.id);
        assert_eq!(last_access_time, Some(1));
        assert_eq!(location.as_deref(), Some("Location 1"));
        assert_eq!(name, "Workspace 1");

        Ok(())
    })
}

#[test]
fn it_does_not_return_workspace() -> Result<()> {
    with_context(|ctx| {
        let FindWorkspaceTestContest { conn } = ctx;

        let record = workspace_record_fixture(Default::default());
        sqlite::insert_workspace(&conn, record.clone())?;

        let workspace = sqlite::find_workspace(&conn, Uuid::nil().as_bytes())?;

        assert!(workspace.is_none());

        Ok(())
    })
}
