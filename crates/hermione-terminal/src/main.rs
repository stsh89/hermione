mod coordinator;
mod forms;
mod handlers;
mod layouts;
mod message;
mod models;
mod params;
mod presenters;
mod providers;
mod router;
mod routes;
mod services;
mod smart_input;
mod themes;
mod widgets;

pub(crate) use handlers::*;
pub(crate) use message::*;
pub(crate) use models::*;
pub(crate) use params::*;
pub(crate) use presenters::*;
pub(crate) use routes::*;

use coordinator::Coordinator;
use hermione_tracing::{NewTracerParameters, Tracer};
use providers::powershell::PowerShellProcess;
use router::TerminalRouter;
use rusqlite::Connection;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use themes::Theme;

type Error = anyhow::Error;
type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let app_path = app_path()?;
    let theme = Theme::parse(include_str!("../themes/github_dark.json"))?;
    let conn = Connection::open(app_path.join("hermione.db3"))?;

    hermione_drive::sqlite::create_workspaces_table_if_not_exists(&conn)?;
    hermione_drive::sqlite::create_commands_table_if_not_exists(&conn)?;
    hermione_drive::sqlite::create_backup_credentials_table_if_not_exists(&conn)?;

    let coordinator = Coordinator {
        database_connection: conn,
        powershell_process: PowerShellProcess::spawn()?,
    };

    let router = TerminalRouter { coordinator, theme };

    let tracer = Tracer::new(NewTracerParameters {
        directory: &app_path,
    });

    let _guard = tracer.init_non_blocking()?;

    if let Err(err) = hermione_tui::run(router) {
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

fn user_path() -> Result<PathBuf> {
    let dir = dirs::home_dir().ok_or(anyhow::Error::msg("Missing terminal app user directory"))?;

    Ok(dir)
}
