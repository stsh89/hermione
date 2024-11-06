mod fixtures;

pub use fixtures::*;
use rusqlite::{Connection, Result};

pub fn count_workspaces(conn: &Connection) -> Result<usize> {
    let count = conn
        .prepare("SELECT COUNT(*) FROM workspaces")?
        .query_row([], |row| row.get(0))?;

    Ok(count)
}
