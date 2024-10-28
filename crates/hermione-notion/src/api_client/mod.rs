pub mod de;

mod headers;
mod request_sender;

use eyre::Result;
use request_sender::RequestSender;
use reqwest::{Response, Url};
use serde::{de::DeserializeOwned, Deserialize};
use std::time::Duration;

const DATABASE_QUERY_PAGE_SIZE: u8 = 100;
const NOTION_BASE_URL: &str = "https://api.notion.com/v1/";
const PAGES_URI: &str = "pages";
const REQUEST_TIMEOUT_IN_SECS: u64 = 5;

pub struct NotionApiClient {
    api_key: Option<String>,
    base_url: Url,
    inner: reqwest::Client,
}

pub struct NewNotionApiClientParameters {
    pub api_key: Option<String>,
    pub base_url_override: Option<String>,
    pub timeout: Duration,
}

pub struct SendParameters<'a> {
    pub api_key_override: Option<&'a str>,
    pub body: Option<serde_json::Value>,
    pub method: Method,
    pub uri: &'a str,
}

#[derive(Debug)]
pub struct Method(reqwest::Method);

#[derive(Debug)]
pub struct QueryDatabaseParameters<'a> {
    pub page_size: u8,
    pub start_cursor: Option<&'a str>,
    pub filter: Option<serde_json::Value>,
    pub api_key_override: Option<&'a str>,
}

#[derive(Deserialize)]
pub struct QueryDatabaseResponse<T> {
    #[serde(rename(deserialize = "results"))]
    pub database_pages: Vec<DatabasePage<T>>,

    pub next_cursor: Option<String>,
}

#[derive(Deserialize)]
pub struct DatabasePage<T> {
    #[serde(rename(deserialize = "id"))]
    pub page_id: String,

    pub properties: T,
}

impl NotionApiClient {
    pub async fn create_database_entry(
        &self,
        database_id: &str,
        properties: serde_json::Value,
    ) -> Result<Response> {
        let body = serde_json::json!({
            "parent": { "database_id": database_id },
            "properties": properties,
        });

        let parameters = SendParameters {
            body: Some(body),
            api_key_override: None,
            uri: PAGES_URI,
            method: Method(reqwest::Method::POST),
        };

        self.execute(parameters).await
    }

    async fn execute(&self, parameters: SendParameters<'_>) -> Result<Response> {
        let SendParameters {
            body,
            uri,
            api_key_override,
            method,
        } = parameters;

        let api_key = api_key_override
            .or(self.api_key.as_deref())
            .ok_or_else(|| eyre::Error::msg("Notion API key is not set"))?;

        let url = self.url(uri)?;

        let mut request_builder = self
            .inner
            .request(method.into(), url)
            .headers(headers::authorization(api_key)?);

        if let Some(body) = body {
            request_builder = request_builder.json(&body);
        }

        let response = RequestSender::new(request_builder).send().await?;

        Ok(response)
    }

    pub async fn query_database<T>(
        &self,
        database_id: &str,
        parameters: QueryDatabaseParameters<'_>,
    ) -> Result<QueryDatabaseResponse<T>>
    where
        T: DeserializeOwned,
    {
        let QueryDatabaseParameters {
            page_size,
            start_cursor,
            filter,
            api_key_override,
        } = parameters;

        let uri = format!("databases/{database_id}/query");

        let mut body = serde_json::json!({
            "page_size": page_size,
        });

        if let Some(start_cursor) = start_cursor {
            body["start_cursor"] = start_cursor.into();
        }

        if let Some(filter) = filter {
            body["filter"] = filter;
        }

        let parameters = SendParameters {
            body: Some(body),
            api_key_override,
            uri: &uri,
            method: Method::post(),
        };

        let response = self.execute(parameters).await?;

        let database_query_result = response
            .json::<QueryDatabaseResponse<T>>()
            .await
            .map_err(eyre::Error::new)?;

        Ok(database_query_result)
    }

    pub fn new(parameters: NewNotionApiClientParameters) -> Result<Self> {
        let NewNotionApiClientParameters {
            timeout,
            api_key,
            base_url_override,
        } = parameters;

        let base_url = base_url(base_url_override)?;
        let headers = headers::default_headers();

        let inner = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(timeout)
            .build()
            .map_err(eyre::Error::new)?;

        Ok(Self {
            inner,
            base_url,
            api_key,
        })
    }

    pub async fn update_database_entry(
        &self,
        entry_id: &str,
        properties: serde_json::Value,
    ) -> Result<Response> {
        let uri = format!("pages/{entry_id}");

        let body = serde_json::json!({
            "properties": properties,
        });

        let parameters = SendParameters {
            body: Some(body),
            api_key_override: None,
            uri: &uri,
            method: Method(reqwest::Method::PATCH),
        };

        self.execute(parameters).await
    }

    fn url(&self, uri: &str) -> Result<Url> {
        self.base_url.join(uri).map_err(|err| eyre::Error::new(err))
    }
}

impl Method {
    pub fn post() -> Self {
        Self(reqwest::Method::POST)
    }
}

impl Default for NewNotionApiClientParameters {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(REQUEST_TIMEOUT_IN_SECS),
            api_key: None,
            base_url_override: None,
        }
    }
}

impl From<Method> for reqwest::Method {
    fn from(method: Method) -> Self {
        method.0
    }
}

impl<'a> Default for QueryDatabaseParameters<'a> {
    fn default() -> Self {
        Self {
            page_size: DATABASE_QUERY_PAGE_SIZE,
            start_cursor: None,
            filter: None,
            api_key_override: None,
        }
    }
}

fn base_url(base_url_override: Option<String>) -> Result<Url> {
    let url = if let Some(base_url) = base_url_override {
        Url::parse(&base_url)
    } else {
        Url::parse(NOTION_BASE_URL)
    };

    url.map_err(|err| eyre::Error::new(err))
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;
    use httpmock::{Method::POST, MockServer};
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Product {
        #[serde(
            rename(deserialize = "Name"),
            deserialize_with = "de::title::deserializer"
        )]
        name: String,

        #[serde(
            rename(deserialize = "Description"),
            deserialize_with = "de::rich_text::deserializer"
        )]
        description: String,
    }

    #[tokio::test]
    async fn it_queries_database() -> Result<()> {
        let mock_server = MockServer::start_async().await;
        let base_url = mock_server.base_url();

        let mock = mock_server.mock(|when, then| {
            when.path("/databases/1111/query").method(POST);
            then.body_from_file("tests/fixtures/database_query.json")
                .status(200);
        });

        let client = NotionApiClient::new(NewNotionApiClientParameters {
            base_url_override: Some(base_url.clone()),
            api_key: Some("".to_string()),
            ..Default::default()
        })?;

        let parameters = QueryDatabaseParameters::default();
        let response = client.query_database::<Product>("1111", parameters).await?;

        assert_eq!(response.database_pages.len(), 1);

        let page = response.database_pages.into_iter().next().unwrap();

        assert_eq!(page.properties.name, "Tuscan kale");
        assert_eq!(page.properties.description, "A dark green leafy vegetable");

        mock.assert_async().await;

        Ok(())
    }
}
