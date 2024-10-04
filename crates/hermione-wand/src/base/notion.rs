use crate::Result;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client as InnerClient, Response, StatusCode, Url,
};
use serde_json::Value;
use std::time::Duration;

const NOTION_HEADER_VERSION_NAME: &str = "notion-version";
const NOTION_HEADER_VERSION_VALUE: &str = "2022-06-28";
const NOTION_BASE_URL: &str = "https://api.notion.com/v1/";
const NOTION_HEADER_CONTENT_TYPE_VALUE: &str = "application/json";

pub struct Client {
    inner: InnerClient,
    base_url: Url,
}

#[derive(Default)]
pub struct PostParameters<'a> {
    pub body: Option<Value>,
    pub uri: &'a str,
    pub api_key: &'a str,
}

impl Client {
    pub fn new() -> Result<Self> {
        let mut headers = HeaderMap::new();

        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static(NOTION_HEADER_CONTENT_TYPE_VALUE),
        );

        headers.insert(
            HeaderName::from_static(NOTION_HEADER_VERSION_NAME),
            HeaderValue::from_static(NOTION_HEADER_VERSION_VALUE),
        );

        let base_url = Url::parse(NOTION_BASE_URL)?;

        let inner = InnerClient::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(3))
            .build()?;

        Ok(Self { inner, base_url })
    }

    pub async fn post(&self, parameters: PostParameters<'_>) -> Result<Value> {
        let PostParameters { body, uri, api_key } = parameters;

        let url = self.base_url.join(uri)?;

        let mut request_builder = self.inner.post(url).header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {api_key}"))?,
        );

        if let Some(body) = body {
            request_builder = request_builder.json(&body);
        }

        let response = request_builder.send().await?;
        let status = response.status();

        if let StatusCode::OK = status {
            return Ok(response.json().await?);
        }

        Err(eyre::eyre!(error_message(response).await?))
    }

    pub fn with_base_url(self, base_url: Url) -> Self {
        Self {
            inner: self.inner,
            base_url,
        }
    }
}

impl<'a> PostParameters<'a> {
    pub fn with_body(self, body: Value) -> Self {
        Self {
            body: Some(body),
            ..self
        }
    }

    pub fn with_uri(self, uri: &'a str) -> Self {
        Self { uri, ..self }
    }

    pub fn with_api_key(self, api_key: &'a str) -> Self {
        Self { api_key, ..self }
    }
}

async fn error_message(response: Response) -> Result<String> {
    let status = response.status().as_u16();
    let response_body: Value = response.json().await?;
    let message = response_body["message"].as_str().unwrap_or_default();

    Ok(serde_json::json!({
        "error": "Notion API request failure",
        "details": {
            "status": status,
            "message": message
        }
    })
    .to_string())
}
