use std::path::Path;

mod core;
pub mod workspaces;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
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

fn connection(dir_path: &Path) -> Result<rusqlite::Connection> {
    let path = dir_path.join("hermione.db3");
    let connection = rusqlite::Connection::open(path).map_err(ErrReport::err_report)?;

    Ok(connection)
}
