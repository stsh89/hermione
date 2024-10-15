use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

mod core;
mod records;

pub mod commands;
pub mod workspaces;

pub type Result<T> = std::result::Result<T, Error>;

const DATABASE_FILE_PATH: &str = "hermione.db3";

pub struct Connection {
    inner: rusqlite::Connection,
    dir_path: PathBuf,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Database(#[from] rusqlite::Error),

    #[error("{0}")]
    FailedPrecondition(String),

    #[error("{0}")]
    Internal(String),

    #[error(transparent)]
    Unknown(#[from] eyre::Report),
}

impl From<hermione_core::Error> for Error {
    fn from(value: hermione_core::Error) -> Self {
        match value {
            hermione_core::Error::FailedPrecondition(msg) => Self::FailedPrecondition(msg),
            hermione_core::Error::Internal(msg) => Self::Internal(msg),
            hermione_core::Error::Unknown(err) => Self::Unknown(err),
        }
    }
}

trait ErrReport {
    fn err_report(self) -> eyre::Error;
}

impl ErrReport for rusqlite::Error {
    fn err_report(self) -> eyre::Error {
        eyre::Error::new(self)
    }
}

impl Deref for Connection {
    type Target = rusqlite::Connection;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Connection {
    pub fn open(dir_path: &Path) -> Result<Self> {
        let path = dir_path.join(DATABASE_FILE_PATH);
        let connection = rusqlite::Connection::open(path).map_err(ErrReport::err_report)?;

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

        Ok(Self {
            inner: connection,
            dir_path: dir_path.to_path_buf(),
        })
    }

    pub fn try_clone(&self) -> Result<Self> {
        let path = self.dir_path.join(DATABASE_FILE_PATH);
        let inner = rusqlite::Connection::open(path).map_err(ErrReport::err_report)?;

        Ok(Self {
            inner,
            dir_path: self.dir_path.clone(),
        })
    }
}
