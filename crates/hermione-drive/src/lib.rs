mod backup;
mod storage;
mod system;

pub use backup::*;
pub use storage::*;
pub use system::*;

use hermione_internals::{file_system::AppLocation, powershell::PowerShellProcess, sqlite};
use rusqlite::Connection;
use std::path::Path;
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};

pub struct Engine {
    pub service_factory: ServiceFactory,
    pub logs_worker_guard: WorkerGuard,
}

pub struct ServiceFactory {
    powershell: PowerShellProcess,
    conn: Connection,
}

impl ServiceFactory {
    pub fn system(&self) -> System {
        System::new(&self.powershell)
    }

    pub fn storage(&self) -> Storage {
        Storage::new(&self.conn)
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
