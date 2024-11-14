mod coordinator;
mod handlers;
mod layouts;
mod message;
mod models;
mod params;
mod providers;
mod router;
mod routes;
mod screen;
mod services;
mod smart_input;
mod themes;
mod tui;
mod widgets;

pub(crate) use handlers::*;
pub(crate) use message::*;
pub(crate) use params::*;
pub(crate) use routes::*;

use coordinator::Coordinator;
use hermione_drive::sqlite;
use providers::powershell::PowerShellProcess;
use router::TerminalRouter;
use rusqlite::Connection;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};

type Error = anyhow::Error;
type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let app_path = app_path()?;
    let conn = Connection::open(app_path.join("hermione.db3"))?;

    sqlite::create_workspaces_table_if_not_exists(&conn)?;
    sqlite::create_commands_table_if_not_exists(&conn)?;
    sqlite::create_backup_credentials_table_if_not_exists(&conn)?;

    let coordinator = Coordinator {
        database_connection: conn,
        powershell_process: PowerShellProcess::spawn()?,
    };

    let router = TerminalRouter {
        coordinator,
        theme: themes::github_dark(),
    };

    let _guard = init_tracing(&app_path)?;

    if let Err(err) = tui::run(router) {
        tracing::error!(error = ?err);
        return Err(err);
    };

    Ok(())
}

/// File system location for the terminal app files
pub fn app_path() -> Result<PathBuf> {
    let is_release = cfg!(not(debug_assertions));

    let mut app_path = if is_release {
        user_path()?
    } else {
        development_path()?
    };

    app_path.push(".hermione");

    if !app_path.try_exists()? {
        fs::create_dir(&app_path)?;
    }

    Ok(app_path.to_path_buf())
}

fn development_path() -> Result<PathBuf> {
    let output = Command::new("cargo")
        .args(["locate-project", "--workspace", "--message-format", "plain"])
        .output()?;

    let project_path = std::str::from_utf8(&output.stdout)?;

    Path::new(project_path)
        .parent()
        .map(|path| path.to_path_buf())
        .ok_or(anyhow::Error::msg(
            "Missing terminal app development directory",
        ))
}

fn init_tracing(directory: &Path) -> Result<WorkerGuard> {
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

fn user_path() -> Result<PathBuf> {
    let dir = dirs::home_dir().ok_or(anyhow::Error::msg("Missing terminal app user directory"))?;

    Ok(dir)
}
