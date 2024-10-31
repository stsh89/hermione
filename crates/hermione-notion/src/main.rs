mod api_client;
mod cli;
mod providers;

use cli::{Cli, CliSubcommand, Run};
use hermione_ops::{
    backup::BackupOperation,
    notion::{
        DeleteCredentialsOperation, GetCredentialsOperation, SaveCredentialsOperation,
        VerifyCredentialsOperation,
    },
};
use hermione_storage::StorageProvider;
use hermione_tracing::{NewTracerParameters, Tracer};
use providers::{
    credentials::NotionCredentialsProvider,
    pages::{NotionDatabasePropertiesProvider, NotionDatabaseProvider},
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

        let notion_provider = NotionDatabaseProvider::new(credentials)?;

        BackupOperation {
            iterate_commands_provider: &notion_provider.commands_iterator(),
            iterate_workspaces_provider: &notion_provider.workspaces_iterator(),
            import_command_provider: &self.storage_provider,
            import_workspace_provider: &self.storage_provider,
            list_commands_provider: &self.storage_provider,
            list_workspaces_provider: &self.storage_provider,
            update_command_provider: &self.storage_provider,
            update_workspace_provider: &self.storage_provider,
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

        let notion_provider = NotionDatabaseProvider::new(credentials)?;

        BackupOperation {
            iterate_commands_provider: &self.storage_provider.commands_iterator(),
            iterate_workspaces_provider: &self.storage_provider.workspaces_iterator(),
            import_command_provider: &notion_provider,
            import_workspace_provider: &notion_provider,
            list_commands_provider: &notion_provider,
            list_workspaces_provider: &notion_provider,
            update_command_provider: &notion_provider,
            update_workspace_provider: &notion_provider,
        }
        .execute()
        .await?;

        Ok(())
    }

    async fn save_credentials(self) -> Result<()> {
        SaveCredentialsOperation {
            save_credentials_provider: &self.credentials_provider,
            get_credentials_provider: &ScreenProvider::new(),
            get_database_properties_provider: &NotionDatabasePropertiesProvider {},
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

        ScreenProvider::new().show_credentials(credentials.into())?;

        Ok(())
    }

    async fn verify_credentials(self) -> Result<()> {
        let credentials = GetCredentialsOperation {
            get_credentials_provider: &self.credentials_provider,
        }
        .execute()?;

        VerifyCredentialsOperation {
            get_database_properties_provider: &NotionDatabasePropertiesProvider {},
        }
        .execute(&credentials)
        .await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let app_path = hermione_storage::app_path()?;

    let credentials_provider = NotionCredentialsProvider::new(app_path.join("notion.json"));
    let connection = StorageProvider::connect(&app_path)?;
    let storage_provider = StorageProvider::new(&connection)?;

    let tracer = Tracer::new(NewTracerParameters {
        directory: &app_path,
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
