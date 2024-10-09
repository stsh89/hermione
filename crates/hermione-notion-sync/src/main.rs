use clap::{Parser, Subcommand};
use hermione_notion::QueryDatabaseParameters;
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

#[tokio::main]
async fn main() -> Result<()> {
    let app_path = hermione_terminal_directory::path()?;

    let cli = Cli::parse();

    let path = app_path.join(SETTINGS_FILE_NAME);

    let result = match cli.command {
        Commands::CreateSettingsFile => create_settings_file(&path).await,
        Commands::DeleteSettingsFile => delete_settings_file(&path),
    };

    if let Err(error) = result {
        eprintln!("{error:?}");
    }

    Ok(())
}

async fn create_settings_file(settings_file_path: &Path) -> Result<()> {
    if settings_file_path.try_exists()? {
        return Err(Error::msg("Settings file already exists"));
    }

    clear_screen();
    let api_key = read_stdin("Enter your Notion API key: ")?;

    clear_screen();
    let workspaces_page_id = read_stdin("Enter your Notion workspaces page ID: ")?;

    let settings = Settings {
        api_key,
        workspaces_page_id,
    };

    clear_screen();
    println!("Settings verification started...");

    let client = hermione_notion::Client::new(hermione_notion::ClientParameters {
        timeout: std::time::Duration::from_secs(10),
        api_key: Some(settings.api_key.clone()),
        base_url_override: None,
    })?;

    client
        .query_database(QueryDatabaseParameters {
            api_key_override: None,
            page_size: 1,
            start_cursor: None,
            database_id: &settings.workspaces_page_id,
        })
        .await?;

    println!("Settings verified!");

    let file = File::create(settings_file_path)?;
    serde_json::to_writer_pretty(file, &settings)?;

    println!("Settings file created: {}", settings_file_path.display());

    Ok(())
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
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
