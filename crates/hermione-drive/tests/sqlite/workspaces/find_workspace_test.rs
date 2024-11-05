use crate::solutions::workspace_record_fixture;
use hermione_drive::sqlite::workspaces::{self, WorkspaceRecord};
use rusqlite::{Connection, Result};
use uuid::Uuid;

struct FindWorkspaceTestContest {
    conn: Connection,
    record: WorkspaceRecord,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(FindWorkspaceTestContest) -> Result<()>,
{
    let conn = Connection::open_in_memory()?;
    workspaces::migrate(&conn)?;

    let record = workspace_record_fixture(Default::default());
    workspaces::insert_workspace(&conn, record.clone())?;

    test_fn(FindWorkspaceTestContest { conn, record })
}

#[test]
fn it_returns_workspace() -> Result<()> {
    with_context(|ctx| {
        let FindWorkspaceTestContest { conn, record } = ctx;

        let Some(WorkspaceRecord {
            id,
            last_access_time,
            location,
            name,
        }) = workspaces::find_workspace(&conn, &record.id)?
        else {
            unreachable!("Expected record to be found")
        };

        assert_eq!(id, record.id);
        assert_eq!(last_access_time, record.last_access_time);
        assert_eq!(location, record.location);
        assert_eq!(name, record.name);

        Ok(())
    })
}

#[test]
fn it_does_not_return_workspace() -> Result<()> {
    with_context(|ctx| {
        let FindWorkspaceTestContest { conn, record: _ } = ctx;

        let workspace = workspaces::find_workspace(&conn, Uuid::nil().as_bytes())?;

        assert!(workspace.is_none());

        Ok(())
    })
}
