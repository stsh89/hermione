mod cli;
mod client;
mod clients;
mod de;
mod providers;

use cli::{Cli, CliSubcommand, Run};
use clients::file_system::FileSystemClient;
use hermione_ops::notion::{
    DeleteCredentialsOperation, ExportOperation, GetCredentialsOperation, ImportOperation,
    SaveCredentialsOperation, VerifyCredentialsOperation,
};
use hermione_storage::StorageProvider;
use hermione_tracing::{NewTracerParameters, Tracer};
use providers::{
    credentials::NotionCredentialsProvider,
    pages::{
        NotionCommandsIteratorProvider, NotionCommandsProvider, NotionProvider,
        NotionWorkspacesIteratorProvider, NotionWorkspacesProvider,
    },
    screen::ScreenProvider,
};

type Result<T> = anyhow::Result<T>;

struct App<'a> {
    credentials_provider: NotionCredentialsProvider,
    storage_provider: StorageProvider<'a>,
}

enum Command {
    DeleteCredentials,
    Export,
    Import,
    SaveCredentials,
    ShowCredentials,
    VerifyCredentials,
}

impl<'a> App<'a> {
    fn delete_credentials(self) -> Result<()> {
        DeleteCredentialsOperation {
            deleter: &self.credentials_provider,
        }
        .execute()?;

        Ok(())
    }

    async fn import(self) -> Result<()> {
        let credentials = GetCredentialsOperation {
            get_credentials_provider: &self.credentials_provider,
        }
        .execute()?;

        let notion_provider = NotionProvider::new(Some(credentials))?;

        let local_commands_provider = &self.storage_provider.commands_backup_provider();
        let local_workspaces_provider = &self.storage_provider.workspaces_backup_provider();
        let notion_commands_provider = &NotionCommandsIteratorProvider::new(&notion_provider);
        let notion_workspaces_provider = &NotionWorkspacesIteratorProvider::new(&notion_provider);

        ImportOperation {
            local_commands_provider,
            notion_commands_provider,
            local_workspaces_provider,
            notion_workspaces_provider,
        }
        .execute()
        .await?;

        Ok(())
    }

    async fn export(&self) -> Result<()> {
        let credentials = GetCredentialsOperation {
            get_credentials_provider: &self.credentials_provider,
        }
        .execute()?;

        let notion_provider = NotionProvider::new(Some(credentials))?;

        let local_commands_provider = &self.storage_provider.commands_backup_provider();
        let local_workspaces_provider = &self.storage_provider.workspaces_backup_provider();
        let notion_commands_provider = &NotionCommandsProvider::new(&notion_provider);
        let notion_workspaces_provider = &NotionWorkspacesProvider::new(&notion_provider);

        ExportOperation {
            local_commands_provider,
            notion_commands_provider,
            local_workspaces_provider,
            notion_workspaces_provider,
        }
        .execute()
        .await?;

        Ok(())
    }

    async fn save_credentials(self) -> Result<()> {
        SaveCredentialsOperation {
            saver: &self.credentials_provider,
            getter: &ScreenProvider::new(),
            verifier: &NotionProvider::new(None)?,
        }
        .execute()
        .await?;

        Ok(())
    }

    fn show_credentials(self) -> Result<()> {
        let credentials = GetCredentialsOperation {
            get_credentials_provider: &self.credentials_provider,
        }
        .execute()?;

        ScreenProvider::new().show_credentials(credentials)?;

        Ok(())
    }

    async fn verify_credentials(self) -> Result<()> {
        VerifyCredentialsOperation {
            get_credentials_provider: &self.credentials_provider,
            verify_credentials_provider: &NotionProvider::new(None)?,
        }
        .execute()
        .await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let app_path = hermione_storage::app_path()?;

    let credentials_provider = NotionCredentialsProvider {
        client: FileSystemClient::new(app_path.join("notion.json")),
    };

    let connection = StorageProvider::connect(&app_path)?;
    let storage_provider = StorageProvider::new(&connection)?;

    let tracer = Tracer::new(NewTracerParameters {
        directory: &app_path,
        filename_prefix: "hermione-notion-logs",
    });

    let app = App {
        credentials_provider,
        storage_provider,
    };

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
