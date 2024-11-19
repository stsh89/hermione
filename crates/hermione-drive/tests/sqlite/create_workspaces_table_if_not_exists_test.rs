use crate::support::{expected_workspaces_table_schema, query_table_schema};
use hermione_drive::providers::sqlite;
use rusqlite::{Connection, Result};

#[test]
fn it_creates_table() -> Result<()> {
    let connection = Connection::open_in_memory()?;

    let schema = query_table_schema(&connection, "workspaces")?;
    assert_eq!(schema, vec![]);

    sqlite::create_workspaces_table_if_not_exists(&connection)?;

    let schema = query_table_schema(&connection, "workspaces")?;
    assert_eq!(schema, expected_workspaces_table_schema());

    Ok(())
}

#[test]
fn it_does_not_fail_if_table_already_exists() -> Result<()> {
    let connection = Connection::open_in_memory()?;

    sqlite::create_workspaces_table_if_not_exists(&connection)?;

    let schema = query_table_schema(&connection, "workspaces")?;
    assert_eq!(schema, expected_workspaces_table_schema());

    sqlite::create_workspaces_table_if_not_exists(&connection)?;

    Ok(())
}
