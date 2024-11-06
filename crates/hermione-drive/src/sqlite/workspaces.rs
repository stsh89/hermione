use rusqlite::{named_params, params, Connection, OptionalExtension, Result};
use uuid::Bytes;

#[derive(Clone)]
pub struct WorkspaceRecord {
    pub id: Bytes,
    pub last_access_time: Option<i64>,
    pub location: Option<String>,
    pub name: String,
}

pub struct ListWorkspacesQuery<'a> {
    pub name_contains: &'a str,
    pub limit: u32,
    pub offset: u32,
}

pub fn create_workspaces_table_if_not_exists(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS workspaces (
            id BLOB PRIMARY KEY,
            last_access_time INTEGER,
            location TEXT,
            name TEXT NOT NULL
        )",
        (),
    )?;

    Ok(())
}

pub fn find_workspace(conn: &Connection, id: &Bytes) -> Result<Option<WorkspaceRecord>> {
    conn.prepare("SELECT id, last_access_time, location, name FROM workspaces WHERE id = ?1")?
        .query_row(params![id], |row| {
            Ok(WorkspaceRecord {
                id: row.get(0)?,
                last_access_time: row.get(1)?,
                location: row.get(2)?,
                name: row.get(3)?,
            })
        })
        .optional()
}

pub fn delete_workspace(conn: &Connection, id: &Bytes) -> Result<usize> {
    conn.prepare("DELETE FROM workspaces WHERE id = ?1")?
        .execute(params![id])
}

pub fn insert_workspace(conn: &Connection, record: WorkspaceRecord) -> Result<usize> {
    let WorkspaceRecord {
        id,
        last_access_time,
        location,
        name,
    } = record;

    conn.prepare(
        "INSERT INTO workspaces (
            id,
            last_access_time,
            location,
            name
        ) VALUES (:id, :last_access_time, :location, :name)",
    )?
    .execute(named_params![
        ":id": id,
        ":last_access_time": last_access_time,
        ":location": location,
        ":name": name
    ])
}

pub fn list_workspaces(
    conn: &Connection,
    query: ListWorkspacesQuery,
) -> Result<Vec<WorkspaceRecord>> {
    let ListWorkspacesQuery {
        name_contains,
        limit,
        offset,
    } = query;

    let name_contains = format!("%{}%", name_contains.to_lowercase());

    let mut statement = conn.prepare(
        "SELECT
            id,
            last_access_time,
            location,
            name
        FROM workspaces
        WHERE
            LOWER(name) LIKE :name_contains
        ORDER BY last_access_time DESC, name ASC
        LIMIT :limit OFFSET :offset",
    )?;

    let records = statement
        .query_map(
            named_params![
                ":name_contains": name_contains,
                ":limit": limit,
                ":offset": offset * limit
            ],
            |row| {
                Ok(WorkspaceRecord {
                    id: row.get(0)?,
                    last_access_time: row.get(1)?,
                    location: row.get(2)?,
                    name: row.get(3)?,
                })
            },
        )?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(records)
}

pub fn refresh_workspace_access_time(conn: &Connection, id: &Bytes, time: i64) -> Result<usize> {
    conn.prepare("UPDATE workspaces SET last_access_time = :time WHERE id = :id")?
        .execute(named_params! {":id": id, ":time": time})
}

pub fn restore_workspaces(conn: &Connection, records: Vec<WorkspaceRecord>) -> Result<()> {
    let mut statement = conn.prepare(
        "INSERT INTO workspaces
        VALUES (:id, :last_access_time, :location, :name)
        ON CONFLICT (id) DO UPDATE SET
            last_access_time = excluded.last_access_time,
            location = excluded.location,
            name = excluded.name",
    )?;

    for record in records {
        let WorkspaceRecord {
            id,
            last_access_time,
            location,
            name,
        } = record;

        statement.execute(named_params![
            ":id": id,
            ":last_access_time": last_access_time,
            ":location": location,
            ":name": name
        ])?;
    }

    Ok(())
}

pub fn update_workspace(conn: &Connection, record: WorkspaceRecord) -> Result<usize> {
    let WorkspaceRecord {
        id,
        last_access_time,
        location,
        name,
    } = record;

    conn.prepare(
        "UPDATE workspaces
        SET
            last_access_time = :last_access_time,
            location = :location,
            name = :name
        WHERE id = :id",
    )?
    .execute(named_params![
        ":id": id,
        ":last_access_time": last_access_time,
        ":location": location,
        ":name": name
    ])
}
