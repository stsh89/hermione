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
    fn execute(&self) -> OperationResult;
}

pub struct App {
    /// Parsed command line arguments.
    cli: Cli,

    /// The path to the directory where all the files related to the Notion Sync app are stored.
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
    App::new()?.enable_tracing()?.run().await
}

impl App {
    fn enable_tracing(self) -> Result<Self> {
        hermione_logs::init(&self.path.join(LOGS_FILE_NAME))?;

        Ok(self)
    }

    fn new() -> Result<App> {
        let cli = Cli::parse();
        let path = hermione_terminal_directory::path()?;

        Ok(App { cli, path })
    }

    async fn run(self) -> Result<()> {
        let operation: Box<dyn Operation> = self.try_into()?;

        operation.execute().await
    }
}

impl TryFrom<App> for Box<dyn Operation> {
    type Error = Error;

    fn try_from(value: App) -> std::result::Result<Self, Self::Error> {
        let App {
            cli: Cli { command },
            path,
        } = value;

        let boxed_operation: Box<dyn Operation> = match command {
            CliCommand::CreateSettingsFile => {
                let operation = operations::create_settings_file::Operation::new(path)?;

                Box::new(operation)
            }
            CliCommand::DeleteSettingsFile => {
                let operation = operations::delete_settings_file::Operation::new(path);

                Box::new(operation)
            }
            CliCommand::Export => {
                let operation = operations::export::Operation::new(path)?;

                Box::new(operation)
            }
            CliCommand::Import => {
                let operation = operations::import::Operation::new(path)?;

                Box::new(operation)
            }
            CliCommand::ShowSettingsFile => {
                let operation = operations::show_settings_file::Operation::new(path);

                Box::new(operation)
            }
            CliCommand::VerifySettingsFile => {
                let operation = operations::verify_settings_file::Operation::new(path);

                Box::new(operation)
            }
        };

        Ok(boxed_operation)
    }
}
