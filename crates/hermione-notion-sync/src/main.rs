mod commands;
mod screen;
mod settings;

use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

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
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let directory_path = hermione_terminal_directory::path()?;

    let result = match cli.command {
        Commands::CreateSettingsFile => create_settings_file(directory_path).await,
        Commands::DeleteSettingsFile => delete_settings_file(directory_path),
        Commands::Export => export(directory_path).await,
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
