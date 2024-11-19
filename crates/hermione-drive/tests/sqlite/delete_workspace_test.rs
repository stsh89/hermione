use crate::support::{count_workspaces, workspace_record_fixture};
use hermione_drive::providers::sqlite::{self, WorkspaceRecord};
use rusqlite::{Connection, Result};
use uuid::Uuid;

struct DeleteWorkspaceTestContext {
    conn: Connection,
    record: WorkspaceRecord,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(DeleteWorkspaceTestContext) -> Result<()>,
{
    let conn = Connection::open_in_memory()?;
    sqlite::create_workspaces_table_if_not_exists(&conn)?;

    let record = workspace_record_fixture(Default::default());
    sqlite::insert_workspace(&conn, record.clone())?;

    test_fn(DeleteWorkspaceTestContext { conn, record })
}

#[test]
fn it_deletes_workspace() -> Result<()> {
    with_context(|ctx| {
        let DeleteWorkspaceTestContext { conn, record } = ctx;

        assert_eq!(count_workspaces(&conn)?, 1);

        let count = sqlite::delete_workspace(&conn, &record.id)?;

        assert_eq!(count, 1);
        assert_eq!(count_workspaces(&conn)?, 0);

        Ok(())
    })
}

#[test]
fn it_does_not_delete_workspace() -> Result<()> {
    with_context(|ctx| {
        let DeleteWorkspaceTestContext { conn, record: _ } = ctx;

        assert_eq!(count_workspaces(&conn)?, 1);

        let count = sqlite::delete_workspace(&conn, Uuid::nil().as_bytes())?;

        assert_eq!(count_workspaces(&conn)?, 1);
        assert_eq!(count, 0);

        Ok(())
    })
}
