mod notion;
mod operations;
mod screen;
mod settings;

use clap::{Parser, Subcommand};
use std::{future::Future, path::PathBuf, pin::Pin};

const LOGS_FILE_NAME: &str = "hermione-notion-sync.logs";

type Error = anyhow::Error;
type OperationResult<'a> = Pin<Box<dyn Future<Output = Result<()>> + 'a>>;
type Result<T> = anyhow::Result<T>;

pub trait Operation {
    // async fn execute(&self) -> Result<()>;
    fn execute(&self) -> OperationResult;
}

pub struct App {
    /// Parsed command line arguments
    cli: Cli,

    /// The path to the directory where the settings file should be found or is actually located.
    path: PathBuf,
}

#[derive(Debug, Parser)]
#[command(about)]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Debug, Subcommand)]
enum CliCommand {
    CreateSettingsFile,
    DeleteSettingsFile,
    Export,
    Import,
    ShowSettingsFile,
    VerifySettingsFile,
}

#[tokio::main]
async fn main() -> Result<()> {
    initialize()?.build_operation()?.execute().await
}

impl App {
    fn build_operation(self) -> Result<Box<dyn Operation>> {
        let boxed_operation: Box<dyn Operation> = match self.cli.command {
            CliCommand::CreateSettingsFile => {
                let operation = operations::create_settings_file::Operation::new(self.path)?;

                Box::new(operation)
            }
            CliCommand::DeleteSettingsFile => {
                let operation = operations::delete_settings_file::Operation::new(self.path);

                Box::new(operation)
            }
            CliCommand::Export => {
                let operation = operations::export::Operation::new(self.path)?;

                Box::new(operation)
            }
            CliCommand::Import => {
                let operation = operations::import::Operation::new(self.path)?;

                Box::new(operation)
            }
            CliCommand::ShowSettingsFile => {
                let operation = operations::show_settings_file::Operation::new(self.path);

                Box::new(operation)
            }
            CliCommand::VerifySettingsFile => {
                let operation = operations::verify_settings_file::Operation::new(self.path);

                Box::new(operation)
            }
        };

        Ok(boxed_operation)
    }
}

pub fn initialize() -> Result<App> {
    let cli = Cli::parse();
    let path = hermione_terminal_directory::path()?;

    hermione_logs::init(&path.join(LOGS_FILE_NAME))?;

    Ok(App { cli, path })
}
