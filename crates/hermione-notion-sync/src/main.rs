mod commands;
mod screen;
mod settings;

use clap::{Parser, Subcommand};
use hermione_coordinator::workspaces::{Dto, ListParameters};
use hermione_notion::{
    json::{Json, PageId, RichText, Title},
    NewClientParameters, QueryDatabaseParameters,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

type Result<T> = anyhow::Result<T>;
type Error = anyhow::Error;

use settings::{NewSettingsParameters, Settings};

const PAGE_SIZE: u32 = 1;

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
        Commands::Export => export(&directory_path).await,
    };

    if let Err(error) = result {
        eprintln!("{error}");
    }

    Ok(())
}

#[derive(Serialize)]
struct RichTextFilter {
    property: String,
    rich_text: RichTextEqualsFilter,
}

#[derive(Serialize)]
struct RichTextEqualsFilter {
    equals: String,
}

#[derive(Deserialize)]
struct QueryDatabaseOutput {
    results: Vec<Json>,
}

async fn export(app_path: &Path) -> Result<()> {
    let settings = Settings::read(app_path)?;

    let coordinator = hermione_coordinator::workspaces::Client::new(app_path)?;
    let mut page_number = 0;

    use hermione_coordinator::workspaces::Operations;
    let workspaces = coordinator.list(ListParameters {
        name_contains: "",
        page_number,
        page_size: PAGE_SIZE,
    })?;

    let notion_client = hermione_notion::Client::new(NewClientParameters {
        api_key: Some(settings.api_key().into()),
        ..Default::default()
    })?;

    let filters: Vec<RichTextFilter> = workspaces
        .iter()
        .map(|workspace| RichTextFilter {
            property: "External ID".to_string(),
            rich_text: RichTextEqualsFilter {
                equals: workspace.id.clone(),
            },
        })
        .collect();

    let filter = serde_json::json!({
        "or": serde_json::json!(filters),
    });

    let json = notion_client
        .query_database(
            settings.workspaces_page_id(),
            QueryDatabaseParameters {
                page_size: workspaces.len() as u8,
                filter: Some(filter),
                ..Default::default()
            },
        )
        .await?;

    let len = workspaces.len();
    let mut created = 0;
    let mut updated = 0;
    println!("Exporting {} workspaces...", workspaces.len());

    println!("{}", json.to_string());

    let empty = &vec![];
    let results = json["results"].as_array().unwrap_or(empty);

    for workspace in workspaces {
        let found = results
            .into_iter()
            .find(|json_value| json_value.rich_text("External ID") == &workspace.id);

        let Some(record) = found else {
            println!("Creating workspace: {}", &workspace.name);
            create_workspace(&settings, &notion_client, workspace).await?;
            created += 1;

            continue;
        };

        if record.title() != &workspace.name
            || record.rich_text("Location") != workspace.location.as_ref().unwrap_or(&String::new())
        {
            println!("Updating workspace: {}", workspace.name);
            update_workspace(record.id(), &notion_client, workspace).await?;
            updated += 1;
        }
    }

    println!(
        "Summary. Total {}. Created {}. Updated {}",
        len, created, updated
    );

    Ok(())
}

async fn create_workspace(
    settings: &Settings,
    notion_client: &hermione_notion::Client,
    workspace: Dto,
) -> Result<()> {
    notion_client
        .create_database_entry(
            settings.workspaces_page_id(),
            serde_json::json!({
                "Name": {"title": [{"text": {"content": workspace.name}}]},
                "External ID": {"rich_text": [{"text": {"content": workspace.id}}]},
                "Location": {"rich_text": [{"text": {"content": workspace.location}}]}
            }),
        )
        .await?;

    Ok(())
}

async fn update_workspace(
    page_id: &str,
    notion_client: &hermione_notion::Client,
    workspace: Dto,
) -> Result<()> {
    notion_client
        .update_database_entry(
            &page_id,
            serde_json::json!({
                "Name": {"title": [{"text": {"content": workspace.name}}]},
                "Location": {"rich_text": [{"text": {"content": workspace.location}}]}
            }),
        )
        .await?;

    Ok(())
}
