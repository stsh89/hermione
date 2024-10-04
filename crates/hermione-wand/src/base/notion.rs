use crate::Result;
use eyre::eyre;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client as InnerClient, Response, StatusCode, Url,
};
use serde_json::Value;
use std::time::Duration;

const NOTION_BASE_URL: &str = "https://api.notion.com/v1/";
const NOTION_HEADER_CONTENT_TYPE_VALUE: &str = "application/json";
const NOTION_HEADER_VERSION_NAME: &str = "notion-version";
const NOTION_HEADER_VERSION_VALUE: &str = "2022-06-28";

pub struct Client {
    base_url: Url,
    inner: InnerClient,
}

pub struct PostParameters<'a> {
    api_key: &'a str,
    body: Option<Value>,
    uri: &'a str,
}

#[derive(Default)]
pub struct PostParametersBuilder<'a> {
    api_key: Option<&'a str>,
    body: Option<Value>,
    uri: Option<&'a str>,
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

        let error_message = api_request_error_message(response).await?;
        let error = eyre!(error_message);

        Err(error.wrap_err("Notion API request error"))
    }

    pub fn with_base_url(self, base_url: Url) -> Self {
        Self {
            inner: self.inner,
            base_url,
        }
    }
}

impl<'a> PostParametersBuilder<'a> {
    pub fn build(self) -> Result<PostParameters<'a>> {
        let api_key = self.api_key.ok_or(eyre!("Notion API key is required"))?;
        let uri = self.uri.ok_or(eyre!("Notion API URI is required"))?;

        Ok(PostParameters {
            api_key,
            body: self.body,
            uri,
        })
    }

    pub fn with_api_key(self, api_key: &'a str) -> Self {
        Self {
            api_key: Some(api_key),
            ..self
        }
    }

    pub fn with_body(self, body: Value) -> Self {
        Self {
            body: Some(body),
            ..self
        }
    }

    pub fn with_uri(self, uri: &'a str) -> Self {
        Self {
            uri: Some(uri),
            ..self
        }
    }
}

impl<'a> PostParameters<'a> {
    pub fn builder() -> PostParametersBuilder<'a> {
        PostParametersBuilder::default()
    }
}

async fn api_request_error_message(response: Response) -> Result<String> {
    let status = response.status().as_u16();
    let response_body: Value = response.json().await?;
    let message = response_body["message"].as_str().unwrap_or_default();

    Ok(serde_json::json!({
        "status": status,
        "message": message
    })
    .to_string())
}
