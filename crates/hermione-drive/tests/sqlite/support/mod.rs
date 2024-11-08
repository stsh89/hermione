mod fixtures;

pub use fixtures::*;
use rusqlite::{params, Connection, Result};

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
