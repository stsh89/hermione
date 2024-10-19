mod notion;
mod operations;
mod screen;
mod settings;

use clap::{Parser, Subcommand};
use hermione_tracing::{NewTracerParameters, Tracer};
use std::{future::Future, path::Path, pin::Pin};

const LOGS_FILE_NAME_PREFIX: &str = "hermione-notion-sync-logs";

type Error = anyhow::Error;
type OperationResult<'a> = Pin<Box<dyn Future<Output = Result<()>> + 'a>>;
type Result<T> = anyhow::Result<T>;

pub trait Operation {
    fn execute(&self) -> OperationResult;
}

struct App<'a> {
    cli: Cli,
    directory: &'a Path,
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
    let directory = hermione_terminal_directory::path()?;
    let cli = Cli::parse();

    let app = App {
        cli,
        directory: &directory,
    };

    let tracer = Tracer::new(NewTracerParameters {
        directory: &directory,
        filename_prefix: LOGS_FILE_NAME_PREFIX,
    });

    let _guard = tracer.init_non_blocking()?;

    if let Err(err) = run(app).await {
        tracing::error!(error = ?err);
        return Err(err);
    }

    Ok(())
}

async fn run(app: App<'_>) -> Result<()> {
    let App {
        cli: Cli { command },
        directory,
    } = app;

    let operation: Box<dyn Operation> = match command {
        CliCommand::CreateSettingsFile => {
            let operation = operations::create_settings_file::Operation::new(directory)?;

            Box::new(operation)
        }
        CliCommand::DeleteSettingsFile => {
            let operation =
                operations::delete_settings_file::Operation::new(directory.to_path_buf());

            Box::new(operation)
        }
        CliCommand::Export => {
            let operation = operations::export::Operation::new(directory)?;

            Box::new(operation)
        }
        CliCommand::Import => {
            let operation = operations::import::Operation::new(directory)?;

            Box::new(operation)
        }
        CliCommand::ShowSettingsFile => {
            let operation = operations::show_settings_file::Operation::new(directory.to_path_buf());

            Box::new(operation)
        }
        CliCommand::VerifySettingsFile => {
            let operation =
                operations::verify_settings_file::Operation::new(directory.to_path_buf());

            Box::new(operation)
        }
    };

    operation.execute().await
}
