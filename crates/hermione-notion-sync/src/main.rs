mod commands;
mod screen;
mod settings;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

const LOGS_FILE_NAME: &str = "hermione-notion-sync.logs";

type Result<T> = anyhow::Result<T>;
type Error = anyhow::Error;

#[derive(Debug, Parser)]
#[command(about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    CreateSettingsFile,
    DeleteSettingsFile,
    Export,
    ShowSettingsFile,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let directory_path = hermione_terminal_directory::path()?;

    hermione_logs::init(&directory_path.join(LOGS_FILE_NAME))?;

    let result = match cli.command {
        Commands::CreateSettingsFile => create_settings_file(directory_path).await,
        Commands::DeleteSettingsFile => delete_settings_file(directory_path),
        Commands::Export => export(directory_path).await,
        Commands::ShowSettingsFile => show_settings_file(directory_path),
    };

    if let Err(error) = result {
        eprintln!("{error}");
    }

    Ok(())
}

async fn create_settings_file(directory_path: PathBuf) -> Result<()> {
    commands::create_settings_file::Command::new(&directory_path)?
        .execute()
        .await
}

fn delete_settings_file(directory_path: PathBuf) -> Result<()> {
    commands::delete_settings_file::Command::new(directory_path).execute()
}

async fn export(directory_path: PathBuf) -> Result<()> {
    commands::export::Command::new(directory_path)?
        .execute()
        .await
}

fn show_settings_file(directory_path: PathBuf) -> Result<()> {
    commands::show_settings_file::Command::new(directory_path).execute()
}
