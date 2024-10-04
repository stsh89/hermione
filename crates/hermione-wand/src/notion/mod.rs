use crate::{
    base::notion::{Client as InnerClient, PostParameters},
    Result,
};
use reqwest::Url;
use serde::Deserialize;
use serde_json::Value;

pub struct Client {
    inner: InnerClient,
    api_key: String,
}

#[derive(Default)]
pub struct NewClientParameters {
    pub api_key: String,
    pub base_url: Option<String>,
}

pub struct QueryDatabaseParameters<'a> {
    pub page_size: u8,
    pub start_cursor: Option<&'a str>,
    pub database_id: &'a str,
}

impl Default for QueryDatabaseParameters<'_> {
    fn default() -> Self {
        Self {
            page_size: 100,
            start_cursor: None,
            database_id: "",
        }
    }
}

impl NewClientParameters {
    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = api_key.to_string();
        self
    }

    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = Some(base_url.to_string());
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryDatabaseOutput {
    pub next_cursor: Option<String>,

    #[serde(rename = "results")]
    pub records: Vec<Value>,
}

impl Client {
    pub fn new(parameters: NewClientParameters) -> Result<Self> {
        let NewClientParameters { api_key, base_url } = parameters;
        let mut inner = InnerClient::new()?;

        if let Some(base_url) = base_url {
            inner = inner.with_base_url(Url::parse(&base_url)?);
        }

        Ok(Self { inner, api_key })
    }

    pub async fn query_database(
        &self,
        parameters: QueryDatabaseParameters<'_>,
    ) -> Result<QueryDatabaseOutput> {
        let QueryDatabaseParameters {
            page_size,
            start_cursor,
            database_id,
        } = parameters;

        let uri = format!("databases/{database_id}/query");

        let mut body = serde_json::json!({
            "page_size": page_size,
        });

        if let Some(start_cursor) = start_cursor {
            body["start_cursor"] = serde_json::json!(start_cursor);
        }

        let parameters = PostParameters::builder()
            .with_uri(&uri)
            .with_api_key(&self.api_key)
            .with_body(body)
            .build()?;

        let response = self.inner.post(parameters).await?;

        let mut output: QueryDatabaseOutput = serde_json::from_value(response)?;

        output.records = output
            .records
            .into_iter()
            .map(|record| record["properties"].clone())
            .collect::<Vec<Value>>();

        Ok(output)
    }
}

impl<'a> QueryDatabaseParameters<'a> {
    pub fn with_database_id(self, database_id: &'a str) -> Self {
        Self {
            database_id,
            ..self
        }
    }
}
