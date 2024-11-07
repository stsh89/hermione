use std::num::NonZeroU32;

use ureq::{Agent, AgentBuilder, Error, Response};

type Result<T> = std::result::Result<T, Error>;

const NOTION_BASE_URL: &str = "https://api.notion.com/v1";

pub struct NotionApiClient {
    agent: Agent,
    base_url_override: Option<String>,
    api_key: String,
}

pub struct NotionApiClientParameters {
    pub base_url_override: Option<String>,
    pub api_key: String,
}

pub struct QueryDatabaseParameters<'a> {
    pub database_id: &'a str,
    pub start_cursor: Option<&'a str>,
    pub page_size: Option<NonZeroU32>,
}

impl NotionApiClient {
    #[allow(clippy::result_large_err)]
    pub fn get_database_properties(&self, database_id: &str) -> Result<Response> {
        let base_url = self.base_url_override.as_deref().unwrap_or(NOTION_BASE_URL);
        let path = format!("{}/databases/{}", base_url, database_id);

        self.agent
            .get(&path)
            .set("Content-Type", "application/json")
            .set("Notion-Version", "2022-06-28")
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .call()
    }

    #[allow(clippy::result_large_err)]
    pub fn query_database(&self, parameters: QueryDatabaseParameters) -> Result<Response> {
        let QueryDatabaseParameters {
            database_id,
            start_cursor,
            page_size,
        } = parameters;

        let page_size = page_size.unwrap_or(NonZeroU32::new(100).unwrap()).get();

        tracing::info!(
            message = "Query Notion database",
            database_id = database_id,
            page_size = page_size,
            start_cursor = start_cursor
        );

        let base_url = self.base_url_override.as_deref().unwrap_or(NOTION_BASE_URL);
        let path = format!("{}/databases/{}/query", base_url, database_id);

        let mut body = serde_json::json!({
            "page_size": page_size,
        });

        if let Some(start_cursor) = start_cursor {
            body["start_cursor"] = start_cursor.into();
        }

        // TODO: process 429 error properly
        self.agent
            .post(&path)
            .set("Content-Type", "application/json")
            .set("Notion-Version", "2022-06-28")
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .send_json(serde_json::json!(body))
    }

    pub fn new(parameters: NotionApiClientParameters) -> Self {
        let NotionApiClientParameters {
            api_key,
            base_url_override,
        } = parameters;

        let agent = AgentBuilder::new().build();

        Self {
            api_key,
            agent,
            base_url_override,
        }
    }
}
