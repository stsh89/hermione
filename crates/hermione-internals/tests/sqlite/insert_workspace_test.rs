use crate::support::count_workspaces;
use hermione_internals::sqlite::{self, WorkspaceRecord};
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
    sqlite::create_workspaces_table_if_not_exists(&conn)?;

    test_fn(InsertWorkspaceTestContext { conn })
}

#[test]
fn it_inserts_workspace() -> Result<()> {
    with_context(|ctx| {
        let InsertWorkspaceTestContext { conn } = ctx;

        assert_eq!(count_workspaces(&conn)?, 0);

        let count = sqlite::insert_workspace(
            &conn,
            WorkspaceRecord {
                id: Uuid::new_v4().into_bytes(),
                last_access_time: None,
                location: None,
                name: "Test workspace".to_string(),
            },
        )?;

        assert_eq!(count, 1);
        assert_eq!(count_workspaces(&conn)?, 1);

        Ok(())
    })
}
