use crate::{
    Client, DatabasePage, NewClientParameters, QueryDatabaseParameters, QueryDatabaseResponse,
};
use hermione_notion_serde::de;
use hermione_ops::{
    commands::{
        Command, FindCommand, ImportCommand, ListAllCommandsInBatches, LoadCommandParameters,
        UpdateCommand,
    },
    notion::{Credentials, VerifyCredentials},
    workspaces::{
        FindWorkspace, ImportWorkspace, ListAllWorkspacesInBatches, LoadWorkspaceParameters,
        UpdateWorkspace, Workspace,
    },
    Error, Result,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::runtime::Runtime;
use uuid::Uuid;

const DEFAULT_PAGE_SIZE: u8 = 100;

pub struct NotionProvider {
    client: Client,
    credentials: Option<Credentials>,
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

pub struct ListDatabasePagesParameters {
    pub database_id: String,
    pub external_ids: Option<Vec<String>>,
    pub page_size: Option<u8>,
    pub api_key: String,
    pub start_cursor: Option<String>,
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

impl NotionProvider {
    fn credentials(&self) -> Result<&Credentials> {
        self.credentials.as_ref().ok_or(Error::FailedPrecondition(
            "Missing Notion credentials".into(),
        ))
    }

    async fn find_command_page(&self, id: Uuid) -> Result<Option<DatabasePage<CommandProperties>>> {
        self.find_page_by_external_id(
            self.credentials()?.commands_page_id(),
            id.to_string().as_str(),
        )
        .await

        // let credentials = self.credentials()?;

        // let query_database_response = self
        //     .list_database_pages::<CommandProperties>(ListDatabasePagesParameters {
        //         database_id: credentials.commands_page_id().into(),
        //         external_ids: Some(vec![id.to_string()]),
        //         page_size: Some(1),
        //         api_key: credentials.api_key().into(),
        //         start_cursor: None,
        //     })
        //     .await?;

        // Ok(query_database_response.database_pages.into_iter().next())
    }

    async fn find_page_by_external_id<T>(
        &self,
        database_id: &str,
        exteranal_id: &str,
    ) -> Result<Option<DatabasePage<T>>>
    where
        T: DeserializeOwned,
    {
        let api_key = self.credentials()?.api_key().into();

        let query_database_response = self
            .list_database_pages(ListDatabasePagesParameters {
                database_id: database_id.into(),
                external_ids: Some(vec![exteranal_id.into()]),
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
        self.find_page_by_external_id(
            self.credentials()?.workspaces_page_id(),
            id.to_string().as_str(),
        )
        .await

        // self
        //     .list_database_pages::<WorkspaceProperties>(ListDatabasePagesParameters {
        //         database_id: credentials.commands_page_id().into(),
        //         external_ids: Some(vec![id.to_string()]),
        //         page_size: Some(1),
        //         api_key: credentials.api_key().into(),
        //         start_cursor: None,
        //     })
        //     .await?;

        // Ok(page.into_iter().next())
    }

    pub fn new(credentials: Option<Credentials>) -> Result<Self> {
        let api_key = credentials
            .as_ref()
            .map(|credentials| credentials.api_key().into());

        let client = Client::new(NewClientParameters {
            api_key,
            ..Default::default()
        })
        .map_err(eyre::Error::new)?;

        Ok(Self {
            client,
            credentials,
        })
    }

    async fn list_database_pages<T>(
        &self,
        parameters: ListDatabasePagesParameters,
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
                .into_iter()
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
                &database_id,
                QueryDatabaseParameters {
                    page_size: page_size.unwrap_or(DEFAULT_PAGE_SIZE),
                    filter,
                    api_key_override: Some(&api_key),
                    start_cursor: start_cursor.as_deref(),
                },
            )
            .await
            .map_err(eyre::Error::new)?;

        Ok(response)
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

impl ImportCommand for NotionProvider {
    fn import_command(&self, command: Command) -> Result<Command> {
        let id = command
            .id()
            .ok_or(Error::DataLoss("Missing command id".into()))?
            .to_string();

        let commands_page_id = self.credentials()?.commands_page_id();

        Runtime::new()?.block_on(async {
            self.client
            .create_database_entry(
                commands_page_id,
                serde_json::json!({
                    "Name": {"title": [{"text": {"content": command.name()}}]},
                    "External ID": {"rich_text": [{"text": {"content": id}}]},
                    "Program": {"rich_text": [{"text": {"content": command.program()}}]},
                    "Workspace ID": {"rich_text": [{"text": {"content": command.workspace_id().to_string()}}]}
                }),
            )
            .await
        }).map_err(eyre::Error::new)?;

        Ok(command)
    }
}

impl ImportWorkspace for NotionProvider {
    fn import_workspace(&self, workspace: Workspace) -> Result<Workspace> {
        let id = workspace
            .id()
            .ok_or(Error::DataLoss("Missing workspace id".into()))?
            .to_string();

        let workspaces_page_id = self.credentials()?.workspaces_page_id();

        Runtime::new()?
            .block_on(async {
                self.client
                    .create_database_entry(
                        workspaces_page_id,
                        serde_json::json!({
                            "Name": {"title": [{"text": {"content": workspace.name()}}]},
                            "External ID": {"rich_text": [{"text": {"content": id}}]},
                            "Location": {"rich_text": [{"text": {"content": workspace.location()}}]}
                        }),
                    )
                    .await
            })
            .map_err(eyre::Error::new)?;

        Ok(workspace)
    }
}

impl FindCommand for NotionProvider {
    fn find_command(&self, id: Uuid) -> Result<Option<Command>> {
        let page = Runtime::new()?.block_on(async { self.find_command_page(id).await })?;

        page.map(|p| p.properties.try_into()).transpose()
    }
}

impl FindWorkspace for NotionProvider {
    fn find_workspace(&self, id: Uuid) -> Result<Option<Workspace>> {
        let page = Runtime::new()?.block_on(async { self.find_workspace_page(id).await })?;

        page.map(|p| p.properties.try_into()).transpose()
    }
}

impl ListAllWorkspacesInBatches for NotionProvider {
    async fn list_all_workspaces_in_batches(
        &self,
        batch_fn: impl Fn(Vec<Workspace>) -> Result<()>,
    ) -> Result<()> {
        let Some(credentials) = self.credentials.as_ref() else {
            return Err(Error::FailedPrecondition(
                "Missing Notion credentials".into(),
            ));
        };

        let mut start_cursor: Option<String> = None;

        loop {
            let query_database_response = self
                .list_database_pages::<WorkspaceProperties>(ListDatabasePagesParameters {
                    database_id: credentials.workspaces_page_id().into(),
                    external_ids: None,
                    page_size: Some(1),
                    api_key: credentials.api_key().into(),
                    start_cursor,
                })
                .await?;

            start_cursor = query_database_response.next_cursor;

            let workspaces = query_database_response
                .database_pages
                .into_iter()
                .map(|page| page.properties.try_into())
                .collect::<Result<Vec<Workspace>>>()?;

            batch_fn(workspaces)?;

            if start_cursor.is_none() {
                break;
            }
        }

        Ok(())
    }
}

impl ListAllCommandsInBatches for NotionProvider {
    async fn list_all_commands_in_batches(
        &self,
        batch_fn: impl Fn(Vec<Command>) -> Result<()>,
    ) -> Result<()> {
        let Some(credentials) = self.credentials.as_ref() else {
            return Err(Error::FailedPrecondition(
                "Missing Notion credentials".into(),
            ));
        };

        let mut start_cursor: Option<String> = None;

        loop {
            let query_database_response = self
                .list_database_pages::<CommandProperties>(ListDatabasePagesParameters {
                    database_id: credentials.commands_page_id().into(),
                    external_ids: None,
                    page_size: Some(1),
                    api_key: credentials.api_key().into(),
                    start_cursor,
                })
                .await?;

            start_cursor = query_database_response.next_cursor;

            let commands = query_database_response
                .database_pages
                .into_iter()
                .map(|page| page.properties.try_into())
                .collect::<Result<Vec<Command>>>()?;

            batch_fn(commands)?;

            if start_cursor.is_none() {
                break;
            }
        }

        Ok(())
    }
}

impl UpdateCommand for NotionProvider {
    fn update_command(&self, command: Command) -> Result<Command> {
        let id = command.try_id()?;

        let page = Runtime::new()?.block_on(async { self.find_command_page(id).await })?;

        let Some(page) = page else {
            return Err(Error::NotFound(format!("Command with ID: {}", id)));
        };

        Runtime::new()?
            .block_on(async {
                self.client
                    .update_database_entry(
                        &page.page_id,
                        serde_json::json!({
                            "Name": {"title": [{"text": {"content": command.name()}}]},
                            "Program": {"rich_text": [{"text": {"content": command.program()}}]}
                        }),
                    )
                    .await
            })
            .map_err(eyre::Error::new)?;

        Ok(command)
    }
}

impl UpdateWorkspace for NotionProvider {
    fn update_workspace(&self, workspace: Workspace) -> Result<Workspace> {
        let id = workspace.try_id()?;

        let page = Runtime::new()?.block_on(async { self.find_workspace_page(id).await })?;

        let Some(page) = page else {
            return Err(Error::NotFound(format!("Command with ID: {}", id)));
        };

        Runtime::new()?
            .block_on(async {
                self.client
                    .update_database_entry(
                        &page.page_id,
                        serde_json::json!({
                            "Name": {"title": [{"text": {"content": workspace.name()}}]},
                            "Location": {"rich_text": [{"text": {"content": workspace.location()}}]}
                        }),
                    )
                    .await
            })
            .map_err(eyre::Error::new)?;

        Ok(workspace)
    }
}

impl VerifyCredentials for NotionProvider {
    async fn verify(&self, credentials: &Credentials) -> Result<()> {
        self.list_database_pages::<CommandProperties>(ListDatabasePagesParameters {
            database_id: credentials.commands_page_id().into(),
            external_ids: None,
            page_size: Some(1),
            api_key: credentials.api_key().into(),
            start_cursor: None,
        })
        .await?;

        self.list_database_pages::<WorkspaceProperties>(ListDatabasePagesParameters {
            database_id: credentials.workspaces_page_id().into(),
            external_ids: None,
            page_size: Some(1),
            api_key: credentials.api_key().into(),
            start_cursor: None,
        })
        .await?;

        Ok(())
    }
}
