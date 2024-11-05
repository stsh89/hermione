use hermione_drive::sqlite::workspaces::{self, WorkspaceRecord};
use rusqlite::{Connection, Result};
use uuid::Uuid;

struct InsertWorkspaceTestContext {
    conn: Connection,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(InsertWorkspaceTestContext) -> Result<()>,
{
    let conn = Connection::open_in_memory()?;
    workspaces::migrate(&conn)?;

    test_fn(InsertWorkspaceTestContext { conn })
}

#[test]
fn it_inserts_workspace() -> Result<()> {
    with_context(|ctx| {
        let InsertWorkspaceTestContext { conn } = ctx;

        assert_eq!(workspaces::count_workspaces(&conn)?, 0);

        let count = workspaces::insert_workspace(
            &conn,
            WorkspaceRecord {
                id: Uuid::new_v4().into_bytes(),
                last_access_time: None,
                location: None,
                name: "Test workspace".to_string(),
            },
        )?;

        assert_eq!(count, 1);
        assert_eq!(workspaces::count_workspaces(&conn)?, 1);

        Ok(())
    })
}
