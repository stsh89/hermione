use rusqlite::{named_params, params, Connection, OptionalExtension, Result};
use uuid::Bytes;

pub struct CommandRecord {
    pub id: Bytes,
    pub last_execute_time: Option<i64>,
    pub name: String,
    pub program: String,
    pub workspace_id: Bytes,
}

pub struct ListCommandsQuery<'a> {
    pub program_contains: &'a str,
    pub workspace_id: Option<Bytes>,
    pub page_number: u32,
    pub page_size: u32,
}

pub fn create_commands_table_if_not_exists(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS commands (
            id BLOB PRIMARY KEY,
            last_execute_time INTEGER,
            name TEXT NOT NULL,
            program TEXT NOT NULL,
            workspace_id BLOB NOT NULL
        )",
        (),
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS
        commands_workspace_id_idx
        ON commands(workspace_id)",
        (),
    )?;

    Ok(())
}

pub fn find_command(conn: &Connection, id: &Bytes) -> Result<Option<CommandRecord>> {
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
    .optional()
}

pub fn delete_command(conn: &Connection, id: &Bytes) -> Result<usize> {
    conn.prepare("DELETE FROM commands WHERE id = ?1")?
        .execute(params![id])
}

pub fn delete_workspace_commands(conn: &Connection, workspace_id: &Bytes) -> Result<usize> {
    conn.prepare("DELETE FROM commands WHERE workspace_id = ?1")?
        .execute(params![workspace_id])
}

pub fn insert_command(conn: &Connection, record: CommandRecord) -> Result<usize> {
    let CommandRecord {
        id,
        last_execute_time,
        name,
        program,
        workspace_id,
    } = record;

    conn.prepare(
        "INSERT INTO workspaces (
            id,
            last_execute_time,
            name,
            program,
            workspace_id
        ) VALUES (:id, :last_execute_time, :name, :program, :workspace_id)",
    )?
    .execute(named_params![
        ":id": id,
        ":last_execute_time": last_execute_time,
        ":name": name,
        ":program": program,
        ":workspace_id": workspace_id
    ])
}

pub fn list_commands(conn: &Connection, query: ListCommandsQuery) -> Result<Vec<CommandRecord>> {
    let ListCommandsQuery {
        program_contains,
        workspace_id,
        page_number,
        page_size,
    } = query;

    let program_contains = format!("%{}%", program_contains.to_lowercase());

    let mut statement = conn.prepare(
        "SELECT
            id,
            last_execute_time,
            name,
            program,
            workspace_id
        FROM commands
        WHERE
            LOWER(name) LIKE :program_contains
            AND (workspace_id = :workspace_id OR :workspace_id IS NULL)
        ORDER BY last_execute_time DESC, program ASC
        LIMIT :limit OFFSET :offset",
    )?;

    let records = statement
        .query_map(
            named_params![
                ":limit": page_size,
                ":offset": page_number * page_size,
                ":program_contains": program_contains,
                ":workspace_id": workspace_id,
            ],
            |row| {
                Ok(CommandRecord {
                    id: row.get(0)?,
                    last_execute_time: row.get(1)?,
                    name: row.get(2)?,
                    program: row.get(3)?,
                    workspace_id: row.get(4)?,
                })
            },
        )?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(records)
}

pub fn restore_commands(conn: &Connection, records: Vec<CommandRecord>) -> Result<()> {
    let mut statement = conn.prepare(
        "INSERT INTO commands
        VALUES (:id, :last_execute_time, :name, :program, :workspace_id)
        ON CONFLICT (id) DO UPDATE SET
            last_execute_time = excluded.last_execute_time,
            name = excluded.name,
            program = excluded.program,
            workspace_id = excluded.workspace_id",
    )?;

    for record in records {
        let CommandRecord {
            id,
            last_execute_time,
            name,
            program,
            workspace_id,
        } = record;

        statement.execute(named_params![
            ":id": id,
            ":last_execute_time": last_execute_time,
            ":name": name,
            ":program": program,
            ":workspace_id": workspace_id
        ])?;
    }

    Ok(())
}

pub fn update_command(conn: &Connection, record: CommandRecord) -> Result<usize> {
    let CommandRecord {
        id,
        last_execute_time,
        name,
        program,
        workspace_id: _,
    } = record;

    conn.prepare(
        "UPDATE workspaces
        SET
            last_execute_time = :last_execute_time,
            name = :name,
            program = :program
        WHERE id = :id",
    )?
    .execute(named_params![
        ":id": id,
        ":last_execute_time": last_execute_time,
        ":name": name,
        ":program": program
    ])
}
