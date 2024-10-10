use crate::{Error, Result};
use reqwest::{
    header::{self, RETRY_AFTER},
    RequestBuilder, StatusCode, Url,
};
use serde_json::Value;
use std::{default, ops::Deref, path::Display, time::Duration};

const NOTION_BASE_URL: &str = "https://api.notion.com/v1/";
const NOTION_HEADER_CONTENT_TYPE_VALUE: &str = "application/json";
const NOTION_HEADER_VERSION_NAME: &str = "notion-version";
const NOTION_HEADER_VERSION_VALUE: &str = "2022-06-28";

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

pub struct Method(reqwest::Method);

pub struct Json(Value);

pub struct QueryDatabaseParameters<'a> {
    pub api_key_override: Option<&'a str>,
    pub page_size: u8,
    pub start_cursor: Option<&'a str>,
    pub filter: Option<Json>,
}

impl Client {
    fn authorization_string(&self, api_key_override: Option<&str>) -> Result<String> {
        let api_key = if let Some(api_key) = api_key_override {
            api_key
        } else {
            self.api_key
                .as_deref()
                .ok_or(eyre::eyre!("Missing Notion API key"))?
        };

        Ok(format!("Bearer {api_key}"))
    }

    fn authorization_header_value(
        &self,
        api_key_override: Option<&str>,
    ) -> Result<header::HeaderValue> {
        let authorization_string = self.authorization_string(api_key_override)?;

        header::HeaderValue::from_str(&authorization_string)
            .map_err(|err| Error::Unknown(eyre::Error::new(err)))
    }

    pub async fn create_database_entry(&self, database_id: &str, properties: Json) -> Result<Json> {
        let uri = "pages";

        let body = Json(serde_json::json!({
            "parent": { "database_id": database_id },
            "properties": properties.value(),
        }));

        let parameters = SendParameters {
            body: Some(body),
            api_key_override: None,
            uri: &uri,
            method: Method(reqwest::Method::POST),
        };

        self.send(parameters).await
    }

    pub async fn query_database(
        &self,
        database_id: &str,
        parameters: QueryDatabaseParameters<'_>,
    ) -> Result<Json> {
        let QueryDatabaseParameters {
            api_key_override,
            page_size,
            start_cursor,
            filter,
        } = parameters;

        let uri = format!("databases/{database_id}/query");

        let mut body = Json(serde_json::json!({
            "page_size": page_size,
        }));

        if let Some(start_cursor) = start_cursor {
            body.set_key("start_cursor", serde_json::json!(start_cursor));
        }

        if let Some(filter) = filter {
            body.set_key("filter", filter.into());
        }

        let parameters = SendParameters {
            body: Some(body),
            api_key_override,
            uri: &uri,
            method: Method(reqwest::Method::POST),
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
        let headers = default_headers();

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

    pub async fn send(&self, parameters: SendParameters<'_>) -> Result<Json> {
        let SendParameters {
            body,
            uri,
            api_key_override,
            method,
        } = parameters;

        let url = self.url(uri)?;

        let mut request_builder = self.inner.request(method.into(), url);

        request_builder = request_builder.header(
            header::AUTHORIZATION,
            self.authorization_header_value(api_key_override)?,
        );

        if let Some(body) = body {
            request_builder = request_builder.json(body.value());
        }

        let response = RequestSender::new(request_builder).send().await?;

        let status = response.status();

        if let StatusCode::OK = status {
            let json = response.json().await?;

            return Ok(Json(json));
        }

        let response_body: Value = response.json().await?;
        let message = response_body["message"].as_str().unwrap_or_default().into();

        Err(Error::ApiError {
            status: status.as_u16(),
            message,
        })
    }

    pub async fn update_database_entry(&self, entry_id: &str, properties: Json) -> Result<Json> {
        let uri = format!("pages/{entry_id}");

        let body = Json(serde_json::json!({
            "properties": properties.value(),
        }));

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
            .map_err(|err| Error::Unknown(eyre::Error::new(err)))
    }
}

struct RequestSender {
    request_builder: RequestBuilder,
}

impl RequestSender {
    fn new(request_builder: RequestBuilder) -> Self {
        Self { request_builder }
    }

    async fn send(self) -> Result<reqwest::Response> {
        let request_builder = self
            .request_builder
            .try_clone()
            .ok_or(Error::Unknown(eyre::Error::msg("No request builder")))?;
        let response = request_builder.send().await?;

        if let StatusCode::TOO_MANY_REQUESTS = response.status() {
            if let Some(value) = response.headers().get(header::RETRY_AFTER) {
                let seconds = value
                    .to_str()
                    .map_err(|err| Error::Unknown(eyre::Error::new(err)))?
                    .parse::<u64>()
                    .map_err(|err| Error::Unknown(eyre::Error::new(err)))?;

                println!(
                    "Notion API is rate limited, retrying in {} seconds",
                    seconds
                );
                tokio::time::sleep(Duration::from_secs(seconds)).await;

                let response = self.request_builder.send().await?;

                return Ok(response);
            }
        }

        Ok(response)
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
            timeout: Duration::from_secs(5),
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

impl Json {
    fn value(&self) -> &Value {
        &self.0
    }

    fn set_key(&mut self, key: &str, value: Value) {
        self.0[key] = value;
    }

    pub fn new(value: Value) -> Self {
        Self(value)
    }

    pub fn results(&self) -> JsonValue {
        JsonValue(&self.0["results"])
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub struct JsonValue<'a>(&'a Value);

impl From<Json> for Value {
    fn from(value: Json) -> Self {
        value.0
    }
}

impl Deref for Json {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct JsonValueIterator<'a> {
    value: &'a JsonValue<'a>,
    index: usize,
}

impl<'a> Iterator for JsonValueIterator<'a> {
    type Item = JsonValue<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index
            < self
                .value
                .0
                .as_array()
                .map(|array| array.len())
                .unwrap_or(0)
        {
            let result = self.value.0.get(self.index).map(JsonValue);
            self.index += 1;
            result
        } else {
            None
        }
    }
}

impl<'a> JsonValue<'a> {
    pub fn iter(&self) -> JsonValueIterator {
        JsonValueIterator {
            value: self,
            index: 0,
        }
    }

    pub fn rich_text(&self, property_name: &str) -> &str {
        self.0["properties"][property_name]["rich_text"][0]["plain_text"]
            .as_str()
            .unwrap_or_default()
    }

    pub fn title(&self) -> &str {
        self.0["properties"]["Name"]["title"][0]["plain_text"]
            .as_str()
            .unwrap_or_default()
    }

    pub fn id(&self) -> &str {
        self.0["id"].as_str().unwrap_or_default()
    }
}

impl<'a> Default for QueryDatabaseParameters<'a> {
    fn default() -> Self {
        Self {
            api_key_override: None,
            page_size: 100,
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

    url.map_err(|err| Error::Unknown(eyre::Error::new(err)))
}

fn default_headers() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();

    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static(NOTION_HEADER_CONTENT_TYPE_VALUE),
    );

    headers.insert(
        header::HeaderName::from_static(NOTION_HEADER_VERSION_NAME),
        header::HeaderValue::from_static(NOTION_HEADER_VERSION_VALUE),
    );

    headers
}
