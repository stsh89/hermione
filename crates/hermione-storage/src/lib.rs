mod impls;

use rusqlite::{params, Connection, Statement};
use std::path::Path;
use uuid::Bytes;

pub struct CommandRecord {
    pub id: Bytes,
    pub last_execute_time: Option<i64>,
    pub name: String,
    pub program: String,
    pub workspace_id: Bytes,
}

pub struct WorkspaceRecord {
    pub id: Bytes,
    pub last_access_time: Option<i64>,
    pub location: Option<String>,
    pub name: String,
}

pub struct StorageProvider {
    connection: Connection,
}

impl StorageProvider {
    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    pub fn insert_command(&self, record: CommandRecord) -> rusqlite::Result<()> {
        self.connection
            .prepare(
                "INSERT INTO commands (
                    id,
                    last_execute_time,
                    name,
                    program,
                    workspace_id
                ) VALUES (?1, ?2, ?3, ?4, ?5)",
            )?
            .execute(params![
                record.id,
                record.last_execute_time,
                record.name,
                record.program,
                record.workspace_id
            ])?;

        Ok(())
    }

    pub fn insert_workspace(&self, record: WorkspaceRecord) -> rusqlite::Result<()> {
        self.connection
            .prepare(
                "INSERT INTO workspaces (
                    id,
                    last_access_time,
                    location,
                    name
                ) VALUES (?1, ?2, ?3, ?4)",
            )?
            .execute(params![
                record.id,
                record.last_access_time,
                record.location,
                record.name
            ])?;

        Ok(())
    }

    fn migrate(&self) -> rusqlite::Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS workspaces (
                id BLOB PRIMARY KEY,
                last_access_time INTEGER,
                location TEXT,
                name TEXT NOT NULL
            )",
            (),
        )?;

        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS commands (
                id BLOB PRIMARY KEY,
                last_execute_time INTEGER,
                name TEXT NOT NULL,
                program TEXT NOT NULL,
                workspace_id BLOB NOT NULL
            )",
            (),
        )?;

        self.connection.execute(
            "CREATE INDEX IF NOT EXISTS
            commands_workspace_id_idx
            ON commands(workspace_id)",
            (),
        )?;

        Ok(())
    }

    pub fn new(file_path: &Path) -> rusqlite::Result<Self> {
        let connection = Connection::open(file_path)?;
        let provider = Self { connection };

        provider.migrate()?;

        Ok(provider)
    }

    pub fn select_command(&self) -> rusqlite::Result<Statement> {
        self.connection.prepare(
            "SELECT
                id,
                last_execute_time,
                name,
                program,
                workspace_id
            FROM commands
            WHERE id = ?1 AND workspace_id = ?2",
        )
    }

    pub fn select_workspace_statement(&self) -> rusqlite::Result<Statement> {
        self.connection.prepare(
            "SELECT
                    id,
                    last_access_time,
                    location,
                    name
                FROM workspaces
                WHERE id = ?1",
        )
    }
}
