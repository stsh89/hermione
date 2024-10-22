use std::path::{Path, PathBuf};

pub mod commands;
pub mod workspaces;

const DATABASE_FILE_PATH: &str = "hermione.db3";

pub struct Connection(PathBuf);

impl Connection {
    pub fn new(dir_path: &Path) -> anyhow::Result<Self> {
        let path = dir_path.join(DATABASE_FILE_PATH);
        let connection = rusqlite::Connection::open(&path)?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS workspaces (
                id BLOB PRIMARY KEY,
                last_access_time INTEGER,
                location TEXT,
                name TEXT NOT NULL
            )",
            (),
        )?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS commands (
                id BLOB PRIMARY KEY,
                last_execute_time INTEGER,
                name TEXT NOT NULL,
                program TEXT NOT NULL,
                workspace_id BLOB NOT NULL
            )",
            (),
        )?;

        connection.execute(
            "CREATE INDEX IF NOT EXISTS
            commands_workspace_id_idx
            ON commands(workspace_id)",
            (),
        )?;

        Ok(Self(path))
    }

    fn open(&self) -> rusqlite::Result<rusqlite::Connection> {
        let connection = rusqlite::Connection::open(self.path())?;

        Ok(connection)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}
