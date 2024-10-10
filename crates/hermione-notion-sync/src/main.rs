mod commands;
mod screen;
mod settings;

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
        Commands::CreateSettingsFile => {
            commands::create_settings_file::Command::new(&directory_path)?
                .execute()
                .await
        }
        Commands::DeleteSettingsFile => {
            commands::delete_settings_file::Command::new(directory_path).execute()
        }
        Commands::Export => {
            commands::export::Command::new(directory_path)?
                .execute()
                .await
        }
    };

    if let Err(error) = result {
        eprintln!("{error}");
    }

    Ok(())
}
