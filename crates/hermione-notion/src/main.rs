mod cli;
mod client;
mod de;
mod provider;
mod screen;

use cli::{Cli, CliSubcommand, Run};
use hermione_ops::{
    backup::BackupOperation,
    notion::{DeleteCredentialsOperation, GetCredentialsOperation, SaveCredentialsOperation},
};
use hermione_storage::{
    database::DatabaseProvider,
    file_system::{FileSystemProvider, NOTION_SYNC_LOGS_FILE_NAME_PREFIX},
};
use hermione_tracing::{NewTracerParameters, Tracer};
use provider::NotionProvider;
use screen::ScreenProvider;

type Result<T> = anyhow::Result<T>;

struct App {
    file_system: FileSystemProvider,
}

enum Command {
    DeleteCredentials,
    Export,
    Import,
    SaveCredentials,
    ShowCredentials,
    VerifyCredentials,
}

impl App {
    fn delete_credentials(self) -> Result<()> {
        DeleteCredentialsOperation {
            deleter: &self.file_system,
        }
        .execute()?;

        Ok(())
    }

    async fn import(self) -> Result<()> {
        let credentials = GetCredentialsOperation {
            getter: &self.file_system,
        }
        .execute()?;

        let notion_provider = NotionProvider::new(Some(credentials))?;
        let database_provider = DatabaseProvider::new(&self.file_system.database_file_path())?;

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
        let credentials = GetCredentialsOperation {
            getter: &self.file_system,
        }
        .execute()?;

        let notion_provider = NotionProvider::new(Some(credentials))?;
        let database_provider = DatabaseProvider::new(&self.file_system.database_file_path())?;

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
            saver: &self.file_system,
            getter: &ScreenProvider::new(),
            verifier: &NotionProvider::new(None)?,
        }
        .execute()
        .await?;

        Ok(())
    }

    fn show_credentials(self) -> Result<()> {
        let creds = GetCredentialsOperation {
            getter: &self.file_system,
        }
        .execute()?;

        let screen = ScreenProvider::new();
        screen.print("Notion API key: ", creds.api_key());
        screen.print("Notion commands page ID: ", creds.commands_page_id());
        screen.print("Notion workspaces page ID: ", creds.workspaces_page_id());

        Ok(())
    }

    async fn verify_credentials(self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let file_system = FileSystemProvider::new().map_err(|err| anyhow::anyhow!(err))?;

    let tracer = Tracer::new(NewTracerParameters {
        directory: file_system.location().into(),
        filename_prefix: NOTION_SYNC_LOGS_FILE_NAME_PREFIX,
    });

    let app = App { file_system };

    let _guard = tracer.init_non_blocking()?;

    if let Err(err) = Cli::run(app).await {
        tracing::error!(error = ?err);
        return Err(err);
    }

    Ok(())
}

impl Run for App {
    type Command = Command;

    async fn run(self, command: Self::Command) -> Result<()> {
        match command {
            Command::DeleteCredentials => self.delete_credentials()?,
            Command::Import => self.import().await?,
            Command::SaveCredentials => self.save_credentials().await?,
            Command::ShowCredentials => self.show_credentials()?,
            Command::Export => self.export().await?,
            Command::VerifyCredentials => self.verify_credentials().await?,
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
            CliSubcommand::VerifyCredentials => Self::VerifyCredentials,
        }
    }
}
