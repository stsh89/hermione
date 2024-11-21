mod backup;
mod clipboard;
mod storage;
mod system;

pub use backup::*;
pub use clipboard::*;
pub use storage::*;
pub use system::*;

use hermione_internals::{file_system::AppLocation, powershell::PowerShellProcess, sqlite};
use rusqlite::Connection;
use std::{num::NonZeroU32, path::Path};
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};

pub struct Engine {
    pub service_factory: ServiceFactory,
    pub logs_worker_guard: WorkerGuard,
}

pub struct NotionBackupBuilderParameters {
    pub page_size: NonZeroU32,
}

pub struct ServiceFactory {
    powershell: PowerShellProcess,
    conn: Connection,
}

pub struct SystemParameters<'a> {
    pub no_exit: bool,
    pub working_directory: Option<&'a str>,
}

impl ServiceFactory {
    pub fn clipboard(&self) -> Clipboard {
        Clipboard {
            process: &self.powershell,
        }
    }

    pub fn system<'a>(&'a self, parameters: SystemParameters<'a>) -> System {
        let SystemParameters {
            no_exit,
            working_directory,
        } = parameters;

        System {
            process: &self.powershell,
            no_exit,
            working_directory,
        }
    }

    pub fn storage(&self) -> Storage {
        Storage { conn: &self.conn }
    }
}

pub fn start() -> anyhow::Result<Engine> {
    let location = AppLocation::locate()?;
    let directory = location.directory();
    let logs_worker_guard = init_tracing(directory)?;
    let powershell = PowerShellProcess::spawn()?;
    let conn = Connection::open(directory.join("hermione.db3"))?;

    sqlite::create_workspaces_table_if_not_exists(&conn)?;
    sqlite::create_commands_table_if_not_exists(&conn)?;
    sqlite::create_backup_credentials_table_if_not_exists(&conn)?;

    Ok(Engine {
        service_factory: ServiceFactory { powershell, conn },
        logs_worker_guard,
    })
}

fn init_tracing(directory: &Path) -> anyhow::Result<WorkerGuard> {
    let file_appender = RollingFileAppender::builder()
        .max_log_files(3)
        .filename_prefix("hermione-logs")
        .rotation(Rotation::HOURLY)
        .build(directory)?;

    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .json()
        .with_writer(non_blocking)
        .init();

    Ok(guard)
}
