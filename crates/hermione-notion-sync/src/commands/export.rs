use crate::{screen, settings::Settings, Result};
use hermione_coordinator::workspaces::{self, Operations};
use hermione_notion::{
    json::{Json, PageId, RichText, Title},
    QueryDatabaseParameters,
};
use serde::Serialize;
use std::path::PathBuf;

const PAGE_SIZE: u32 = 100;

pub struct Command {
    settings: Settings,
    notion_client: hermione_notion::Client,
    workspaces_coordinator: hermione_coordinator::workspaces::Client,
    total_workspaces: u32,
    created_workspaces: u32,
    updated_workspaces: u32,
    exported_workspaces: u32,
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

#[derive(Debug)]
struct NotionWorkspace {
    external_id: String,
    location: String,
    name: String,
    id: String,
}

impl Command {
    fn local_workspaces(&self, page_number: u32) -> Result<Vec<workspaces::Dto>> {
        let workspaces = self
            .workspaces_coordinator
            .list(workspaces::ListParameters {
                name_contains: "",
                page_number,
                page_size: PAGE_SIZE,
            })?;

        Ok(workspaces)
    }

    pub fn new(directory_path: PathBuf) -> Result<Self> {
        let settings = Settings::read(&directory_path)?;

        let notion_client = hermione_notion::Client::new(hermione_notion::NewClientParameters {
            api_key: Some(settings.api_key().into()),
            ..Default::default()
        })?;

        let workspaces_coordinator = workspaces::Client::new(&directory_path)?;

        Ok(Self {
            settings,
            notion_client,
            workspaces_coordinator,
            total_workspaces: 0,
            created_workspaces: 0,
            updated_workspaces: 0,
            exported_workspaces: 0,
        })
    }

    pub async fn execute(mut self) -> Result<()> {
        let mut page_number = 0;

        loop {
            let local_workspaces = self.local_workspaces(page_number)?;
            self.total_workspaces += local_workspaces.len() as u32;

            if local_workspaces.is_empty() {
                break;
            }

            let remote_workspaces = self.remote_workspaces(&local_workspaces).await?;
            self.sync_workspaces(local_workspaces, remote_workspaces)
                .await?;

            page_number += 1;
        }

        screen::clear_and_reset_cursor();
        screen::print("Export summary:");
        screen::print(&format!("Total workspaces: {}", self.total_workspaces));
        screen::print(&format!("Created workspaces: {}", self.created_workspaces));
        screen::print(&format!("Updated workspaces: {}", self.updated_workspaces));

        Ok(())
    }

    async fn remote_workspaces(
        &self,
        workspaces: &[workspaces::Dto],
    ) -> Result<Vec<NotionWorkspace>> {
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

        let json = self
            .notion_client
            .query_database(
                self.settings.workspaces_page_id(),
                QueryDatabaseParameters {
                    page_size: workspaces.len() as u8,
                    filter: Some(filter),
                    ..Default::default()
                },
            )
            .await?;

        let results = json["results"].as_array();

        let Some(results) = results else {
            return Ok(Vec::new());
        };

        let workspaces = results.iter().map(Into::into).collect();

        Ok(workspaces)
    }

    async fn update_remote_workspace(
        &mut self,
        local_workspace: workspaces::Dto,
        remote_workspace: &NotionWorkspace,
    ) -> Result<()> {
        self.notion_client
            .update_database_entry(
                &remote_workspace.id,
                serde_json::json!({
                    "Name": {"title": [{"text": {"content": local_workspace.name}}]},
                    "Location": {"rich_text": [{"text": {"content": local_workspace.location}}]}
                }),
            )
            .await?;

        self.updated_workspaces += 1;

        Ok(())
    }

    async fn create_remote_workspace(&mut self, local_workspace: workspaces::Dto) -> Result<()> {
        self.notion_client
            .create_database_entry(
                self.settings.workspaces_page_id(),
                serde_json::json!({
                    "Name": {"title": [{"text": {"content": local_workspace.name}}]},
                    "External ID": {"rich_text": [{"text": {"content": local_workspace.id}}]},
                    "Location": {"rich_text": [{"text": {"content": local_workspace.location}}]}
                }),
            )
            .await?;

        self.created_workspaces += 1;
        Ok(())
    }

    async fn sync_workspaces(
        &mut self,
        local_workspaces: Vec<workspaces::Dto>,
        remote_workspaces: Vec<NotionWorkspace>,
    ) -> Result<()> {
        for local_workspace in local_workspaces {
            self.exported_workspaces += 1;

            screen::clear_and_reset_cursor();
            screen::print(&format!(
                "Syncing {}/{} workspaces...",
                self.exported_workspaces, self.total_workspaces,
            ));

            let remote_workspace = remote_workspaces
                .iter()
                .find(|remote_workspace| remote_workspace.external_id == local_workspace.id);

            if let Some(remote_workspace) = remote_workspace {
                if remote_workspace == &local_workspace {
                    continue;
                }

                self.update_remote_workspace(local_workspace, remote_workspace)
                    .await?;
            } else {
                self.create_remote_workspace(local_workspace).await?;
            }
        }

        Ok(())
    }
}

impl From<&Json> for NotionWorkspace {
    fn from(json: &Json) -> Self {
        NotionWorkspace {
            id: json.id().into(),
            external_id: json.rich_text("External ID").into(),
            location: json.rich_text("Location").into(),
            name: json.title().into(),
        }
    }
}

impl PartialEq<workspaces::Dto> for NotionWorkspace {
    fn eq(&self, other: &workspaces::Dto) -> bool {
        self.name == other.name && self.location == other.location.as_deref().unwrap_or_default()
    }
}
