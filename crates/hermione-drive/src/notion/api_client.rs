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
