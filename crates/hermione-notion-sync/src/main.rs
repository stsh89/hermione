use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

type Result<T> = anyhow::Result<T>;
type Error = anyhow::Error;

const SETTINGS_FILE_NAME: &str = "notion-sync.json";

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
}

#[derive(Serialize, Deserialize)]
struct Settings {
    api_key: String,
    workspaces_page_id: String,
}

fn main() -> Result<()> {
    let app_path = hermione_terminal_directory::path()?;

    let cli = Cli::parse();

    let path = app_path.join(SETTINGS_FILE_NAME);

    match cli.command {
        Commands::CreateSettingsFile => create_settings_file(&path),
        Commands::DeleteSettingsFile => delete_settings_file(&path),
    }
}

fn create_settings_file(settings_file_path: &Path) -> Result<()> {
    if settings_file_path.try_exists()? {
        return Err(Error::msg("Settings file already exists"));
    }

    let api_key = read_stdin("Enter your Notion API key: ")?;
    let workspaces_page_id = read_stdin("Enter your Notion workspaces page ID: ")?;
    let settings = Settings {
        api_key,
        workspaces_page_id,
    };

    let file = File::create(settings_file_path)?;
    serde_json::to_writer_pretty(file, &settings)?;

    Ok(())
}

fn delete_settings_file(settings_file_path: &Path) -> Result<()> {
    if !settings_file_path.try_exists()? {
        return Ok(());
    }

    std::fs::remove_file(settings_file_path)?;

    Ok(())
}

fn read_stdin(title: &str) -> Result<String> {
    use std::io::Write;

    let mut buf = String::new();
    print!("{title}");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut buf)?;

    Ok(buf.trim().to_string())
}
