mod cli;
mod screen;

use cli::{Cli, CliSubcommand, Run};
use hermione_notion::NotionProvider;
use hermione_ops::{
    backup::BackupOperation,
    notion::{DeleteCredentialsOperation, GetCredentialsOperation, SaveCredentialsOperation},
};
use hermione_storage::{database::DatabaseProvider, file_system::FileSystemProvider};
use hermione_tracing::{NewTracerParameters, Tracer};
use screen::ScreenProvider;
use std::path::Path;

const LOGS_FILE_NAME_PREFIX: &str = "hermione-notion-sync-logs";

type Result<T> = anyhow::Result<T>;

struct App<'a> {
    /// Directory for storing file with credentials
    file_storage_location: &'a Path,
}

enum Command {
    DeleteCredentials,
    Export,
    Import,
    SaveCredentials,
    ShowCredentials,
}

impl<'a> App<'a> {
    fn delete_credentials(self) -> Result<()> {
        DeleteCredentialsOperation {
            deleter: &FileSystemProvider::new(self.file_storage_location),
        }
        .execute()?;

        Ok(())
    }

    fn file_system_provider(&self) -> FileSystemProvider {
        FileSystemProvider::new(self.file_storage_location)
    }

    async fn import(self) -> Result<()> {
        let file_system_provider = self.file_system_provider();

        let credentials = GetCredentialsOperation {
            getter: &file_system_provider,
        }
        .execute()?;

        let notion_provider = NotionProvider::new(Some(credentials))?;
        let database_provider = DatabaseProvider::new(&file_system_provider.database_file_path())?;

        BackupOperation {
            commands: &notion_provider,
            remote_commands: &database_provider,
            remote_workspaces: &database_provider,
            workspaces: &notion_provider,
        }
        .execute()
        .await?;

        Ok(())
    }

    async fn export(&self) -> Result<()> {
        let file_system_provider = self.file_system_provider();

        let credentials = GetCredentialsOperation {
            getter: &file_system_provider,
        }
        .execute()?;

        let notion_provider = NotionProvider::new(Some(credentials))?;
        let database_provider = DatabaseProvider::new(&file_system_provider.database_file_path())?;

        BackupOperation {
            commands: &database_provider,
            remote_commands: &notion_provider,
            remote_workspaces: &notion_provider,
            workspaces: &database_provider,
        }
        .execute()
        .await?;

        Ok(())
    }

    async fn save_credentials(self) -> Result<()> {
        SaveCredentialsOperation {
            saver: &self.file_system_provider(),
            getter: &ScreenProvider::new(),
            verifier: &NotionProvider::new(None)?,
        }
        .execute()
        .await?;

        Ok(())
    }

    fn show_credentials(self) -> Result<()> {
        let creds = GetCredentialsOperation {
            getter: &self.file_system_provider(),
        }
        .execute()?;

        let screen = ScreenProvider::new();
        screen.print("Notion API key: ", creds.api_key());
        screen.print("Notion commands page ID: ", creds.commands_page_id());
        screen.print("Notion workspaces page ID: ", creds.workspaces_page_id());

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let directory = hermione_terminal_directory::path()?;
    let app = App {
        file_storage_location: &directory,
    };

    let tracer = Tracer::new(NewTracerParameters {
        directory: &directory,
        filename_prefix: LOGS_FILE_NAME_PREFIX,
    });

    let _guard = tracer.init_non_blocking()?;

    if let Err(err) = Cli::run(app).await {
        tracing::error!(error = ?err);
        return Err(err);
    }

    Ok(())
}

impl Run for App<'_> {
    type Command = Command;

    async fn run(self, command: Self::Command) -> Result<()> {
        match command {
            Command::DeleteCredentials => self.delete_credentials()?,
            Command::Import => self.import().await?,
            Command::SaveCredentials => self.save_credentials().await?,
            Command::ShowCredentials => self.show_credentials()?,
            Command::Export => self.export().await?,
        }

        Ok(())
    }
}

impl From<Cli> for Command {
    fn from(value: Cli) -> Self {
        match value.subcommand {
            CliSubcommand::Export => Self::Export,
            CliSubcommand::DeleteCredentials => Self::DeleteCredentials,
            CliSubcommand::Import => Self::Import,
            CliSubcommand::SaveCredentials => Self::SaveCredentials,
            CliSubcommand::ShowCredentials => Self::ShowCredentials,
        }
    }
}
