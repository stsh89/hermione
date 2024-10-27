use crate::client::{
    Client, DatabasePage, NewClientParameters, QueryDatabaseParameters, QueryDatabaseResponse,
};
use crate::de;
use hermione_ops::backup::{Import, Iterate, ListByIds, Update};
use hermione_ops::commands::LoadCommandParameters;
use hermione_ops::workspaces::LoadWorkspaceParameters;
use hermione_ops::{
    commands::Command,
    notion::{Credentials, VerifyCredentials},
    workspaces::Workspace,
    Error, Result,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

const DEFAULT_PAGE_SIZE: u8 = 100;

pub struct NotionProvider {
    client: Client,
    credentials: Option<Credentials>,
}

pub struct NotionCommandsProvider<'a> {
    inner: &'a NotionProvider,
}

pub struct NotionCommandsIteratorProvider<'a> {
    inner: &'a NotionProvider,
    state: RwLock<IteratorState>,
}

pub struct NotionWorkspacesProvider<'a> {
    inner: &'a NotionProvider,
}

pub struct NotionWorkspacesIteratorProvider<'a> {
    inner: &'a NotionProvider,
    state: RwLock<IteratorState>,
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

pub struct ListDatabasePagesParameters<'a> {
    pub database_id: &'a str,
    pub external_ids: Option<&'a [Uuid]>,
    pub page_size: Option<u8>,
    pub api_key: &'a str,
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

impl NotionProvider {
    fn credentials(&self) -> Result<&Credentials> {
        self.credentials.as_ref().ok_or(Error::FailedPrecondition(
            "Missing Notion credentials".into(),
        ))
    }

    async fn find_command_page(&self, id: Uuid) -> Result<Option<DatabasePage<CommandProperties>>> {
        self.find_page_by_external_id(self.credentials()?.commands_page_id(), id)
            .await
    }

    async fn find_page_by_external_id<T>(
        &self,
        database_id: &str,
        exteranal_id: Uuid,
    ) -> Result<Option<DatabasePage<T>>>
    where
        T: DeserializeOwned,
    {
        let api_key = self.credentials()?.api_key();

        let query_database_response = self
            .list_database_pages(ListDatabasePagesParameters {
                database_id,
                external_ids: Some(&[exteranal_id]),
                page_size: Some(1),
                api_key,
                start_cursor: None,
            })
            .await?;

        Ok(query_database_response.database_pages.into_iter().next())
    }

    async fn find_workspace_page(
        &self,
        id: Uuid,
    ) -> Result<Option<DatabasePage<WorkspaceProperties>>> {
        self.find_page_by_external_id(self.credentials()?.workspaces_page_id(), id)
            .await
    }

    pub fn new(credentials: Option<Credentials>) -> Result<Self> {
        let api_key = credentials
            .as_ref()
            .map(|credentials| credentials.api_key().into());

        let client = Client::new(NewClientParameters {
            api_key,
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
            api_key,
            start_cursor,
        } = parameters;
        let mut filter = None;

        if let Some(external_ids) = external_ids {
            let filters: Vec<RichTextFilter> = external_ids
                .iter()
                .map(|id| RichTextFilter {
                    property: "External ID".to_string(),
                    rich_text: RichTextEqualsFilter {
                        equals: id.to_string(),
                    },
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
                    api_key_override: Some(api_key),
                    start_cursor,
                },
            )
            .await?;

        Ok(response)
    }
}

impl<'a> NotionCommandsProvider<'a> {
    pub fn new(provider: &'a NotionProvider) -> Self {
        Self { inner: provider }
    }
}

impl<'a> NotionCommandsIteratorProvider<'a> {
    pub fn new(provider: &'a NotionProvider) -> Self {
        Self {
            inner: provider,
            state: RwLock::new(IteratorState {
                next_cursor: None,
                is_done: false,
            }),
        }
    }
}

impl<'a> NotionWorkspacesProvider<'a> {
    pub fn new(provider: &'a NotionProvider) -> Self {
        Self { inner: provider }
    }
}

impl<'a> NotionWorkspacesIteratorProvider<'a> {
    pub fn new(provider: &'a NotionProvider) -> Self {
        Self {
            inner: provider,
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

impl Import for NotionCommandsProvider<'_> {
    type Entity = Command;

    async fn import(&self, command: Command) -> Result<Command> {
        let id = command
            .id()
            .ok_or(Error::DataLoss("Missing command id".into()))?
            .to_string();

        let commands_page_id = self.inner.credentials()?.commands_page_id();

        self.inner.client
            .create_database_entry(
                commands_page_id,
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

impl<'a> Import for NotionWorkspacesProvider<'a> {
    type Entity = Workspace;

    async fn import(&self, workspace: Workspace) -> Result<Workspace> {
        let id = workspace
            .id()
            .ok_or(Error::DataLoss("Missing workspace id".into()))?
            .to_string();

        let workspaces_page_id = self.inner.credentials()?.workspaces_page_id();

        self.inner
            .client
            .create_database_entry(
                workspaces_page_id,
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

impl<'a> Iterate for NotionCommandsIteratorProvider<'a> {
    type Entity = Command;

    async fn iterate(&self) -> Result<Option<Vec<Command>>> {
        let Some(credentials) = self.inner.credentials.as_ref() else {
            return Err(Error::FailedPrecondition(
                "Missing Notion credentials".into(),
            ));
        };

        let mut state = self.state.write().await;

        if state.is_done {
            return Ok(None);
        }

        let query_database_response = self
            .inner
            .list_database_pages::<CommandProperties>(ListDatabasePagesParameters {
                database_id: credentials.commands_page_id(),
                external_ids: None,
                page_size: Some(DEFAULT_PAGE_SIZE),
                api_key: credentials.api_key(),
                start_cursor: state.next_cursor.as_deref(),
            })
            .await?;

        let commands = query_database_response
            .database_pages
            .into_iter()
            .map(|page| page.properties.try_into())
            .collect::<Result<Vec<Command>>>()?;

        if commands.is_empty() {
            state.is_done = true;

            return Ok(None);
        }

        state.next_cursor = query_database_response.next_cursor;

        Ok(Some(commands))
    }
}

impl<'a> Iterate for NotionWorkspacesIteratorProvider<'a> {
    type Entity = Workspace;

    async fn iterate(&self) -> Result<Option<Vec<Workspace>>> {
        let Some(credentials) = self.inner.credentials.as_ref() else {
            return Err(Error::FailedPrecondition(
                "Missing Notion credentials".into(),
            ));
        };

        let mut state = self.state.write().await;

        if state.is_done {
            return Ok(None);
        }

        let query_database_response = self
            .inner
            .list_database_pages::<WorkspaceProperties>(ListDatabasePagesParameters {
                database_id: credentials.workspaces_page_id(),
                external_ids: None,
                page_size: Some(DEFAULT_PAGE_SIZE),
                api_key: credentials.api_key(),
                start_cursor: state.next_cursor.as_deref(),
            })
            .await?;

        let workspaces = query_database_response
            .database_pages
            .into_iter()
            .map(|page| page.properties.try_into())
            .collect::<Result<Vec<Workspace>>>()?;

        if workspaces.is_empty() {
            state.is_done = true;

            return Ok(None);
        }

        state.next_cursor = query_database_response.next_cursor;

        Ok(Some(workspaces))
    }
}

impl<'a> ListByIds for NotionCommandsProvider<'a> {
    type Entity = Command;

    async fn list_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Command>> {
        let Some(credentials) = self.inner.credentials.as_ref() else {
            return Err(Error::FailedPrecondition(
                "Missing Notion credentials".into(),
            ));
        };

        let pages = self
            .inner
            .list_database_pages::<CommandProperties>(ListDatabasePagesParameters {
                database_id: credentials.commands_page_id(),
                external_ids: Some(&ids),
                page_size: Some(ids.len().try_into().map_err(eyre::Error::new)?),
                api_key: credentials.api_key(),
                start_cursor: None,
            })
            .await?
            .database_pages
            .into_iter()
            .map(|page| page.properties.try_into())
            .collect::<Result<Vec<Command>>>()?;

        Ok(pages)
    }
}

impl<'a> ListByIds for NotionWorkspacesProvider<'a> {
    type Entity = Workspace;

    async fn list_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Workspace>> {
        let Some(credentials) = self.inner.credentials.as_ref() else {
            return Err(Error::FailedPrecondition(
                "Missing Notion credentials".into(),
            ));
        };

        let pages = self
            .inner
            .list_database_pages::<WorkspaceProperties>(ListDatabasePagesParameters {
                database_id: credentials.workspaces_page_id(),
                external_ids: Some(&ids),
                page_size: Some(ids.len().try_into().map_err(eyre::Error::new)?),
                api_key: credentials.api_key(),
                start_cursor: None,
            })
            .await?
            .database_pages
            .into_iter()
            .map(|page| page.properties.try_into())
            .collect::<Result<Vec<Workspace>>>()?;

        Ok(pages)
    }
}

impl<'a> Update for NotionCommandsProvider<'a> {
    type Entity = Command;

    async fn update(&self, command: Command) -> Result<Command> {
        let id = command.try_id()?;

        let page = self.inner.find_command_page(id).await?;

        let Some(page) = page else {
            return Err(Error::NotFound(format!("Command with ID: {}", id)));
        };

        self.inner
            .client
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

impl<'a> Update for NotionWorkspacesProvider<'a> {
    type Entity = Workspace;

    async fn update(&self, workspace: Workspace) -> Result<Workspace> {
        let id = workspace.try_id()?;

        let page = self.inner.find_workspace_page(id).await?;

        let Some(page) = page else {
            return Err(Error::NotFound(format!("Command with ID: {}", id)));
        };

        self.inner
            .client
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

impl VerifyCredentials for NotionProvider {
    async fn verify(&self, credentials: &Credentials) -> Result<()> {
        self.list_database_pages::<CommandProperties>(ListDatabasePagesParameters {
            database_id: credentials.commands_page_id(),
            external_ids: None,
            page_size: Some(1),
            api_key: credentials.api_key(),
            start_cursor: None,
        })
        .await?;

        self.list_database_pages::<WorkspaceProperties>(ListDatabasePagesParameters {
            database_id: credentials.workspaces_page_id(),
            external_ids: None,
            page_size: Some(1),
            api_key: credentials.api_key(),
            start_cursor: None,
        })
        .await?;

        Ok(())
    }
}
