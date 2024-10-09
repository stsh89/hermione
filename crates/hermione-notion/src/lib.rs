use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client as InnerClient, StatusCode, Url,
};
use serde_json::Value;
use std::time::Duration;

const NOTION_BASE_URL: &str = "https://api.notion.com/v1/";
const NOTION_HEADER_CONTENT_TYPE_VALUE: &str = "application/json";
const NOTION_HEADER_VERSION_NAME: &str = "notion-version";
const NOTION_HEADER_VERSION_VALUE: &str = "2022-06-28";

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Notion API request error: {status}, {message}")]
    ApiError { status: u16, message: String },

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[error(transparent)]
    Unknown(#[from] eyre::Report),
}

pub struct Client {
    base_url: Url,
    inner: InnerClient,
    api_key: Option<String>,
}

pub struct ClientParameters {
    pub timeout: Duration,
    pub api_key: Option<String>,
    pub base_url_override: Option<String>,
}

pub struct QueryDatabaseParameters<'a> {
    pub api_key_override: Option<&'a str>,
    pub page_size: u8,
    pub start_cursor: Option<&'a str>,
}

pub struct PostParameters<'a> {
    pub api_key_override: Option<&'a str>,
    pub body: Option<Value>,
    pub uri: &'a str,
}

impl Client {
    pub async fn query_database(
        &self,
        database_id: &str,
        parameters: QueryDatabaseParameters<'_>,
    ) -> Result<Value> {
        let QueryDatabaseParameters {
            api_key_override,
            page_size,
            start_cursor,
        } = parameters;

        let uri = format!("databases/{database_id}/query");

        let mut body = serde_json::json!({
            "page_size": page_size,
        });

        if let Some(start_cursor) = start_cursor {
            body["start_cursor"] = serde_json::json!(start_cursor);
        }

        let parameters = PostParameters {
            body: Some(body),
            api_key_override,
            uri: &uri,
        };

        self.post(parameters).await
    }

    pub fn new(parameters: ClientParameters) -> Result<Self> {
        let ClientParameters {
            timeout,
            api_key,
            base_url_override,
        } = parameters;

        let mut headers = HeaderMap::new();

        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static(NOTION_HEADER_CONTENT_TYPE_VALUE),
        );

        headers.insert(
            HeaderName::from_static(NOTION_HEADER_VERSION_NAME),
            HeaderValue::from_static(NOTION_HEADER_VERSION_VALUE),
        );

        let base_url = if let Some(base_url) = base_url_override {
            Url::parse(&base_url).map_err(eyre::Error::new)?
        } else {
            Url::parse(NOTION_BASE_URL).map_err(eyre::Error::new)?
        };

        let inner = InnerClient::builder()
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

    pub async fn post(&self, parameters: PostParameters<'_>) -> Result<Value> {
        let PostParameters {
            body,
            uri,
            api_key_override,
        } = parameters;

        let url = self.base_url.join(uri).map_err(eyre::Error::new)?;

        let api_key = if let Some(api_key) = api_key_override {
            api_key
        } else {
            self.api_key
                .as_deref()
                .ok_or(eyre::eyre!("Missing Notion API key"))?
        };

        let mut request_builder = self.inner.post(url).header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {api_key}")).map_err(eyre::Error::new)?,
        );

        if let Some(body) = body {
            request_builder = request_builder.json(&body);
        }

        let response = request_builder.send().await?;
        let status = response.status();

        if let StatusCode::OK = status {
            return Ok(response.json().await?);
        }

        let response_body: Value = response.json().await?;
        let message = response_body["message"].as_str().unwrap_or_default().into();

        Err(Error::ApiError {
            status: status.as_u16(),
            message,
        })
    }
}

impl Default for ClientParameters {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(3),
            api_key: None,
            base_url_override: None,
        }
    }
}

impl<'a> Default for QueryDatabaseParameters<'a> {
    fn default() -> Self {
        Self {
            api_key_override: None,
            page_size: 100,
            start_cursor: None,
        }
    }
}
