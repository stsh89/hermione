use rusqlite::{named_params, params, Connection, OptionalExtension, Result};

pub struct BackupCredentialsRecord {
    pub id: String,
    pub secrets: String,
}

pub fn create_backup_credentials_table_if_not_exists(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS backup_credentials (
            id TEXT PRIMARY KEY,
            secrets TEXT NOT NULL
        )",
        (),
    )?;

    Ok(())
}

pub fn find_backup_credentials(conn: &Connection, id: &str) -> Result<Option<BackupCredentialsRecord>> {
    conn.prepare(
        "SELECT
            id,
            secrets
        FROM backup_credentials
        WHERE id = ?1",
    )?
    .query_row(params![id], |row| {
        Ok(BackupCredentialsRecord {
            id: row.get(0)?,
            secrets: row.get(1)?,
        })
    })
    .optional()
}

pub fn delete_backup_credentials(conn: &Connection, id: &str) -> Result<usize> {
    conn.prepare("DELETE FROM backup_credentials WHERE id = ?1")?
        .execute(params![id])
}

pub fn insert_backup_credentials(conn: &Connection, record: BackupCredentialsRecord) -> Result<usize> {
    let BackupCredentialsRecord {
        id,
        secrets,
    } = record;

    conn.prepare(
        "INSERT INTO backup_credentials (
            id,
            secrets
        ) VALUES (:id, :secrets)",
    )?
    .execute(named_params![
        ":id": id,
        ":secrets": secrets,
    ])
}

pub fn list_backup_credentials(conn: &Connection) -> Result<Vec<BackupCredentialsRecord>> {
    let mut statement = conn.prepare(
        "SELECT
            id,
            secrets
        FROM backup_credentials
        ORDER BY id ASC",
    )?;

    let records = statement
        .query_map(
            [],
            |row| {
                Ok(BackupCredentialsRecord {
                    id: row.get(0)?,
                    secrets: row.get(1)?,
                })
            },
        )?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(records)
}

pub fn update_backup_credentials(conn: &Connection, record: BackupCredentialsRecord) -> Result<usize> {
    let BackupCredentialsRecord {
        id,
        secrets,
    } = record;

    conn.prepare(
        "UPDATE backup_credentials
        SET
            secrets = :secrets
        WHERE id = :id",
    )?
    .execute(named_params![
        ":id": id,
        ":secrets": secrets
    ])
}
