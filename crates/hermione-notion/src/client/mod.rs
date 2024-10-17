mod headers;
mod request_sender;

use crate::{json::Json, Error, Result};
use request_sender::RequestSender;
use reqwest::{StatusCode, Url};
use std::time::Duration;

const DATABASE_QUERY_PAGE_SIZE: u8 = 100;
const NOTION_BASE_URL: &str = "https://api.notion.com/v1/";
const PAGES_URI: &str = "pages";
const REQUEST_TIMEOUT_IN_SECS: u64 = 5;

pub struct Client {
    api_key: Option<String>,
    base_url: Url,
    inner: reqwest::Client,
}

pub struct NewClientParameters {
    pub api_key: Option<String>,
    pub base_url_override: Option<String>,
    pub timeout: Duration,
}

pub struct SendParameters<'a> {
    pub api_key_override: Option<&'a str>,
    pub body: Option<Json>,
    pub method: Method,
    pub uri: &'a str,
}

#[derive(Debug)]
pub struct Method(reqwest::Method);

#[derive(Debug)]
pub struct QueryDatabaseParameters<'a> {
    pub page_size: u8,
    pub start_cursor: Option<&'a str>,
    pub filter: Option<Json>,
}

impl Client {
    #[tracing::instrument(skip(self, properties))]
    pub async fn create_database_entry(&self, database_id: &str, properties: Json) -> Result<Json> {
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

        self.send(parameters).await
    }

    #[tracing::instrument(skip(self, parameters))]
    pub async fn query_database(
        &self,
        database_id: &str,
        parameters: QueryDatabaseParameters<'_>,
    ) -> Result<Json> {
        let QueryDatabaseParameters {
            page_size,
            start_cursor,
            filter,
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
            api_key_override: None,
            uri: &uri,
            method: Method::post(),
        };

        self.send(parameters).await
    }

    pub fn new(parameters: NewClientParameters) -> Result<Self> {
        let NewClientParameters {
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

    #[tracing::instrument(skip_all)]
    pub async fn send(&self, parameters: SendParameters<'_>) -> Result<Json> {
        let SendParameters {
            body,
            uri,
            api_key_override,
            method,
        } = parameters;

        let api_key = api_key_override
            .or(self.api_key.as_deref())
            .ok_or_else(|| {
                eyre::Error::new(Error::invalid_argument("Notion API key is not set"))
            })?;

        let url = self.url(uri)?;

        let mut request_builder = self
            .inner
            .request(method.into(), url)
            .headers(headers::authorization(api_key)?);

        if let Some(body) = body {
            request_builder = request_builder.json(&body);
        }

        let response = RequestSender::new(request_builder).send().await?;
        let status = response.status();

        if let StatusCode::OK = status {
            let json = response.json().await?;

            return Ok(json);
        }

        let response_body: Json = response.json().await?;
        let message = response_body["message"].as_str().unwrap_or_default().into();

        Err(Error::ApiError {
            status: status.as_u16(),
            message,
        })
    }

    #[tracing::instrument(skip_all)]
    pub async fn update_database_entry(&self, entry_id: &str, properties: Json) -> Result<Json> {
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

        self.send(parameters).await
    }

    fn url(&self, uri: &str) -> Result<Url> {
        self.base_url
            .join(uri)
            .map_err(|err| Error::Unexpected(eyre::Error::new(err)))
    }
}

impl Method {
    pub fn post() -> Self {
        Self(reqwest::Method::POST)
    }
}

impl Default for NewClientParameters {
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
        }
    }
}

fn base_url(base_url_override: Option<String>) -> Result<Url> {
    let url = if let Some(base_url) = base_url_override {
        Url::parse(&base_url)
    } else {
        Url::parse(NOTION_BASE_URL)
    };

    url.map_err(|err| Error::Unexpected(eyre::Error::new(err)))
}
