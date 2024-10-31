use crate::api_client::{
    de, DatabasePage, NewNotionApiClientParameters, NotionApiClient, QueryDatabaseParameters,
    QueryDatabaseResponse,
};
use hermione_ops::backup::{
    BckImportCommand, BckImportWorkspace, BckIterateCommands, BckIterateWorkspaces,
    BckListCommands, BckListWorkspaces, BckUpdateCommand, BckUpdateWorkspace,
};
use hermione_ops::commands::{CommandId, LoadCommandParameters};
use hermione_ops::notion::{ApiKey, DatabaseId, DatabaseProperty, GetDatabaseProperties};
use hermione_ops::workspaces::{LoadWorkspaceParameters, WorkspaceId};
use hermione_ops::{commands::Command, notion::Credentials, workspaces::Workspace, Error, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::sync::RwLock;

const DEFAULT_PAGE_SIZE: u8 = 100;

pub struct NotionDatabaseProvider {
    client: NotionApiClient,
    credentials: Credentials,
}

pub struct NotionDatabasePropertiesProvider;

pub struct NotionCommandsDatabaseIterator<'a> {
    client: &'a NotionApiClient,
    credentials: &'a Credentials,
    state: RwLock<IteratorState>,
}

pub struct NotionWorkspacesDatabaseIterator<'a> {
    client: &'a NotionApiClient,
    credentials: &'a Credentials,
    state: RwLock<IteratorState>,
}

#[derive(Serialize)]
struct RichTextFilter<'a> {
    property: String,
    rich_text: RichTextEqualsFilter<'a>,
}

#[derive(Serialize)]
struct RichTextEqualsFilter<'a> {
    equals: &'a str,
}

pub struct ListDatabasePagesParameters<'a> {
    pub database_id: &'a str,
    pub external_ids: Option<Vec<String>>,
    pub page_size: Option<u8>,
    pub start_cursor: Option<&'a str>,
}

#[derive(Deserialize)]
pub struct CommandProperties {
    #[serde(
        rename(deserialize = "Name"),
        deserialize_with = "de::title::deserializer"
    )]
    pub name: String,

    #[serde(
        rename(deserialize = "External ID"),
        deserialize_with = "de::rich_text::deserializer"
    )]
    pub external_id: String,

    #[serde(
        rename(deserialize = "Workspace ID"),
        deserialize_with = "de::rich_text::deserializer"
    )]
    pub workspace_id: String,

    #[serde(
        rename(deserialize = "Program"),
        deserialize_with = "de::rich_text::deserializer"
    )]
    pub program: String,
}

#[derive(Deserialize)]
pub struct WorkspaceProperties {
    #[serde(
        rename(deserialize = "Name"),
        deserialize_with = "de::title::deserializer"
    )]
    pub name: String,

    #[serde(
        rename(deserialize = "External ID"),
        deserialize_with = "de::rich_text::deserializer"
    )]
    pub external_id: String,

    #[serde(
        rename(deserialize = "Location"),
        deserialize_with = "de::rich_text::deserializer"
    )]
    pub location: String,
}

struct IteratorState {
    next_cursor: Option<String>,
    is_done: bool,
}

impl NotionDatabaseProvider {
    pub fn commands_iterator(&self) -> NotionCommandsDatabaseIterator {
        NotionCommandsDatabaseIterator {
            client: &self.client,
            credentials: &self.credentials,
            state: RwLock::new(IteratorState {
                next_cursor: None,
                is_done: false,
            }),
        }
    }

    async fn find_command_page(
        &self,
        id: &CommandId,
    ) -> Result<Option<DatabasePage<CommandProperties>>> {
        self.find_page_by_external_id(self.credentials.commands_database_id(), &id.to_string())
            .await
    }

    async fn find_page_by_external_id<T>(
        &self,
        database_id: &str,
        exteranal_id: &str,
    ) -> Result<Option<DatabasePage<T>>>
    where
        T: DeserializeOwned,
    {
        let query_database_response = self
            .list_database_pages(ListDatabasePagesParameters {
                database_id,
                external_ids: Some(vec![exteranal_id.to_string()]),
                page_size: Some(1),
                start_cursor: None,
            })
            .await?;

        Ok(query_database_response.database_pages.into_iter().next())
    }

    async fn find_workspace_page(
        &self,
        id: &WorkspaceId,
    ) -> Result<Option<DatabasePage<WorkspaceProperties>>> {
        self.find_page_by_external_id(self.credentials.workspaces_database_id(), &id.to_string())
            .await
    }

    pub fn new(credentials: Credentials) -> Result<Self> {
        let client = NotionApiClient::new(NewNotionApiClientParameters {
            api_key: Some(credentials.api_key().to_string()),
            ..Default::default()
        })?;

        Ok(Self {
            client,
            credentials,
        })
    }

    async fn list_database_pages<T>(
        &self,
        parameters: ListDatabasePagesParameters<'_>,
    ) -> Result<QueryDatabaseResponse<T>>
    where
        T: DeserializeOwned,
    {
        let ListDatabasePagesParameters {
            external_ids,
            page_size,
            database_id,
            start_cursor,
        } = parameters;
        let mut filter = None;

        if let Some(external_ids) = external_ids {
            let filters: Vec<RichTextFilter> = external_ids
                .iter()
                .map(|id| RichTextFilter {
                    property: "External ID".to_string(),
                    rich_text: RichTextEqualsFilter { equals: id },
                })
                .collect();

            filter = Some(serde_json::json!({
                "or": serde_json::json!(filters),
            }));
        }

        let response = self
            .client
            .query_database(
                database_id,
                QueryDatabaseParameters {
                    page_size: page_size.unwrap_or(DEFAULT_PAGE_SIZE),
                    filter,
                    start_cursor,
                    ..Default::default()
                },
            )
            .await?;

        Ok(response)
    }

    pub fn workspaces_iterator(&self) -> NotionWorkspacesDatabaseIterator {
        NotionWorkspacesDatabaseIterator {
            client: &self.client,
            credentials: &self.credentials,
            state: RwLock::new(IteratorState {
                next_cursor: None,
                is_done: false,
            }),
        }
    }
}

impl TryFrom<CommandProperties> for Command {
    type Error = Error;

    fn try_from(value: CommandProperties) -> Result<Self> {
        let CommandProperties {
            name,
            external_id,
            program,
            workspace_id,
        } = value;

        Ok(Command::load(LoadCommandParameters {
            id: external_id.parse().map_err(eyre::Error::new)?,
            last_execute_time: None,
            program,
            name,
            workspace_id: workspace_id.parse().map_err(eyre::Error::new)?,
        }))
    }
}

impl TryFrom<WorkspaceProperties> for Workspace {
    type Error = Error;

    fn try_from(value: WorkspaceProperties) -> Result<Self> {
        let WorkspaceProperties {
            name,
            external_id,
            location,
        } = value;

        Ok(Workspace::load(LoadWorkspaceParameters {
            id: external_id.parse().map_err(eyre::Error::new)?,
            last_access_time: None,
            location: Some(location),
            name,
        }))
    }
}

impl BckImportCommand for NotionDatabaseProvider {
    async fn bck_import_command(&self, command: Command) -> Result<Command> {
        let id = command.id().to_string();
        let page_id = self.credentials.commands_database_id();

        self.client
            .create_database_entry(
                page_id,
                serde_json::json!({
                    "Name": {"title": [{"text": {"content": command.name()}}]},
                    "External ID": {"rich_text": [{"text": {"content": id}}]},
                    "Program": {"rich_text": [{"text": {"content": command.program()}}]},
                    "Workspace ID": {"rich_text": [{"text": {"content": command.workspace_id().to_string()}}]}
                }),
            )
            .await?;

        Ok(command)
    }
}

impl BckImportWorkspace for NotionDatabaseProvider {
    async fn bck_import_workspace(&self, workspace: Workspace) -> Result<Workspace> {
        let id = workspace.id().to_string();
        let page_id = self.credentials.workspaces_database_id();

        self.client
            .create_database_entry(
                page_id,
                serde_json::json!({
                    "Name": {"title": [{"text": {"content": workspace.name()}}]},
                    "External ID": {"rich_text": [{"text": {"content": id}}]},
                    "Location": {"rich_text": [{"text": {"content": workspace.location()}}]}
                }),
            )
            .await?;

        Ok(workspace)
    }
}

impl<'a> BckIterateCommands for NotionCommandsDatabaseIterator<'a> {
    async fn bck_iterate_commands(&self) -> Result<Option<Vec<Command>>> {
        let mut state = self.state.write().await;

        if state.is_done {
            return Ok(None);
        }

        let query_database_response = self
            .client
            .query_database::<CommandProperties>(
                self.credentials.commands_database_id(),
                QueryDatabaseParameters {
                    page_size: DEFAULT_PAGE_SIZE,
                    filter: None,
                    start_cursor: state.next_cursor.as_deref(),
                    ..Default::default()
                },
            )
            .await?;

        let commands = query_database_response
            .database_pages
            .into_iter()
            .map(|page| page.properties.try_into())
            .collect::<Result<Vec<Command>>>()?;

        state.next_cursor = query_database_response.next_cursor;

        if state.next_cursor.is_none() {
            state.is_done = true;
        }

        if commands.is_empty() {
            Ok(None)
        } else {
            Ok(Some(commands))
        }
    }
}

impl<'a> BckIterateWorkspaces for NotionWorkspacesDatabaseIterator<'a> {
    async fn bck_iterate_workspaces(&self) -> Result<Option<Vec<Workspace>>> {
        let mut state = self.state.write().await;

        if state.is_done {
            return Ok(None);
        }

        let query_database_response = self
            .client
            .query_database::<WorkspaceProperties>(
                self.credentials.workspaces_database_id(),
                QueryDatabaseParameters {
                    page_size: DEFAULT_PAGE_SIZE,
                    filter: None,
                    start_cursor: state.next_cursor.as_deref(),
                    ..Default::default()
                },
            )
            .await?;

        let workspaces = query_database_response
            .database_pages
            .into_iter()
            .map(|page| page.properties.try_into())
            .collect::<Result<Vec<Workspace>>>()?;

        state.next_cursor = query_database_response.next_cursor;

        if state.next_cursor.is_none() {
            state.is_done = true;
        }

        if workspaces.is_empty() {
            Ok(None)
        } else {
            Ok(Some(workspaces))
        }
    }
}

impl BckListCommands for NotionDatabaseProvider {
    async fn bck_list_commands(&self, ids: Vec<&CommandId>) -> Result<Vec<Command>> {
        self.list_database_pages::<CommandProperties>(ListDatabasePagesParameters {
            database_id: self.credentials.commands_database_id(),
            external_ids: Some(ids.iter().map(|id| id.to_string()).collect()),
            page_size: Some(ids.len().try_into().map_err(eyre::Error::new)?),
            start_cursor: None,
        })
        .await?
        .database_pages
        .into_iter()
        .map(|page| page.properties.try_into())
        .collect::<Result<Vec<Command>>>()
    }
}

impl BckListWorkspaces for NotionDatabaseProvider {
    async fn bck_list_workspaces(&self, ids: Vec<&WorkspaceId>) -> Result<Vec<Workspace>> {
        self.list_database_pages::<WorkspaceProperties>(ListDatabasePagesParameters {
            database_id: self.credentials.workspaces_database_id(),
            external_ids: Some(ids.iter().map(|id| id.to_string()).collect()),
            page_size: Some(ids.len().try_into().map_err(eyre::Error::new)?),
            start_cursor: None,
        })
        .await?
        .database_pages
        .into_iter()
        .map(|page| page.properties.try_into())
        .collect::<Result<Vec<Workspace>>>()
    }
}

impl BckUpdateCommand for NotionDatabaseProvider {
    async fn bck_update_command(&self, command: Command) -> Result<Command> {
        let page = self.find_command_page(command.id()).await?;

        let Some(page) = page else {
            return Err(Error::NotFound(format!(
                "Command with ID: {}",
                **command.id()
            )));
        };

        self.client
            .update_database_entry(
                &page.page_id,
                serde_json::json!({
                    "Name": {"title": [{"text": {"content": command.name()}}]},
                    "Program": {"rich_text": [{"text": {"content": command.program()}}]}
                }),
            )
            .await?;

        Ok(command)
    }
}

impl BckUpdateWorkspace for NotionDatabaseProvider {
    async fn bck_update_workspace(&self, workspace: Workspace) -> Result<Workspace> {
        let page = self.find_workspace_page(workspace.id()).await?;

        let Some(page) = page else {
            return Err(Error::NotFound(format!(
                "Command with ID: {}",
                **workspace.id()
            )));
        };

        self.client
            .update_database_entry(
                &page.page_id,
                serde_json::json!({
                    "Name": {"title": [{"text": {"content": workspace.name()}}]},
                    "Location": {"rich_text": [{"text": {"content": workspace.location()}}]}
                }),
            )
            .await?;

        Ok(workspace)
    }
}

impl GetDatabaseProperties for NotionDatabasePropertiesProvider {
    async fn get_database_properties(
        &self,
        api_key: &ApiKey,
        database_id: &DatabaseId,
    ) -> Result<Vec<DatabaseProperty>> {
        let client = NotionApiClient::new(NewNotionApiClientParameters {
            api_key: Some(api_key.to_string()),
            ..Default::default()
        })?;

        let response = client.get_database_properties(database_id).await?;
        let body: serde_json::Value = response.json().await.map_err(eyre::Error::new)?;
        let properties = body["properties"].as_object();

        let Some(properties) = properties else {
            return Err(Error::FailedPrecondition(
                "Can't get Notion page properties".into(),
            ));
        };

        let properties = properties
            .into_iter()
            .map(|(name, values)| {
                Ok(DatabaseProperty {
                    name: name.to_string(),
                    kind: values["type"].as_str().unwrap_or_default().parse()?,
                })
            })
            .collect::<Result<Vec<DatabaseProperty>>>()?;

        Ok(properties)
    }
}
