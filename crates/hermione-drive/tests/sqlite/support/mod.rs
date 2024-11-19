mod fixtures;

pub use fixtures::*;
use hermione_drive::providers::sqlite::{CommandRecord, WorkspaceRecord};
use rusqlite::{params, Connection, Result};
use uuid::Bytes;

#[derive(Debug, PartialEq)]
pub struct ColumnInfo {
    pub name: String,
    pub type_name: String,
    pub not_null: bool,
    pub default_value: Option<String>,
    pub primary_key: bool,
}

pub fn count_workspaces(conn: &Connection) -> Result<usize> {
    let count = conn
        .prepare("SELECT COUNT(*) FROM workspaces")?
        .query_row([], |row| row.get(0))?;

    Ok(count)
}

pub fn query_workspace(conn: &Connection, id: &Bytes) -> Result<WorkspaceRecord> {
    conn.prepare("SELECT id, last_access_time, location, name FROM workspaces WHERE id = ?1")?
        .query_row(params![id], |row| {
            Ok(WorkspaceRecord {
                id: row.get(0)?,
                last_access_time: row.get(1)?,
                location: row.get(2)?,
                name: row.get(3)?,
            })
        })
}

pub fn query_command(conn: &Connection, id: &Bytes) -> Result<CommandRecord> {
    conn.prepare(
        "SELECT
            id,
            last_execute_time,
            name,
            program,
            workspace_id
        FROM commands
        WHERE id = ?1",
    )?
    .query_row(params![id], |row| {
        Ok(CommandRecord {
            id: row.get(0)?,
            last_execute_time: row.get(1)?,
            name: row.get(2)?,
            program: row.get(3)?,
            workspace_id: row.get(4)?,
        })
    })
}

pub fn query_table_schema(connection: &Connection, table_name: &str) -> Result<Vec<ColumnInfo>> {
    connection
        .prepare("SELECT * FROM pragma_table_info(?1)")?
        .query_map(params![table_name], |row| {
            Ok(ColumnInfo {
                name: row.get(1)?,
                type_name: row.get(2)?,
                not_null: row.get(3)?,
                default_value: row.get(4)?,
                primary_key: row.get(5)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()
}

pub fn expected_workspaces_table_schema() -> Vec<ColumnInfo> {
    vec![
        ColumnInfo {
            name: "id".to_string(),
            type_name: "BLOB".to_string(),
            not_null: false,
            default_value: None,
            primary_key: true,
        },
        ColumnInfo {
            name: "last_access_time".to_string(),
            type_name: "INTEGER".to_string(),
            not_null: false,
            default_value: None,
            primary_key: false,
        },
        ColumnInfo {
            name: "location".to_string(),
            type_name: "TEXT".to_string(),
            not_null: false,
            default_value: None,
            primary_key: false,
        },
        ColumnInfo {
            name: "name".to_string(),
            type_name: "TEXT".to_string(),
            not_null: true,
            default_value: None,
            primary_key: false,
        },
    ]
}
