use crate::{Error, Result};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

const NOTION_HEADER_CONTENT_TYPE_VALUE: &str = "application/json";
const NOTION_HEADER_VERSION_NAME: &str = "notion-version";
const NOTION_HEADER_VERSION_VALUE: &str = "2022-06-28";

pub fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static(NOTION_HEADER_CONTENT_TYPE_VALUE),
    );

    headers.insert(
        HeaderName::from_static(NOTION_HEADER_VERSION_NAME),
        HeaderValue::from_static(NOTION_HEADER_VERSION_VALUE),
    );

    headers
}

pub fn authorization(api_key: &str) -> Result<HeaderMap> {
    let value = authorization_header_value(api_key)?;

    Ok(HeaderMap::from_iter(vec![(AUTHORIZATION, value)]))
}

fn authorization_header_value(api_key: &str) -> Result<HeaderValue> {
    let value_string = format!("Bearer {api_key}");

    HeaderValue::from_str(&value_string).map_err(Error::unexpected)
}
