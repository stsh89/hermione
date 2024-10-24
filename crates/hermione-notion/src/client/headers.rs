use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, InvalidHeaderValue, AUTHORIZATION, CONTENT_TYPE,
};

const CONTENT_TYPE_JSON: &str = "application/json";
const VERSION_NAME: &str = "notion-version";
const VERSION_VALUE: &str = "2022-06-28";

pub fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, HeaderValue::from_static(CONTENT_TYPE_JSON));

    headers.insert(
        HeaderName::from_static(VERSION_NAME),
        HeaderValue::from_static(VERSION_VALUE),
    );

    headers
}

pub fn authorization(api_key: &str) -> Result<HeaderMap, InvalidHeaderValue> {
    let value = authorization_header_value(api_key)?;

    Ok(HeaderMap::from_iter(vec![(AUTHORIZATION, value)]))
}

fn authorization_header_value(api_key: &str) -> Result<HeaderValue, InvalidHeaderValue> {
    let value_string = format!("Bearer {api_key}");

    HeaderValue::from_str(&value_string)
}
