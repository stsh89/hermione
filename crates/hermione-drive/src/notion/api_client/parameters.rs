use std::num::NonZeroU32;
use serde_json::Value as Json;

pub struct CreateDatabaseEntryParameters<'a> {
    pub database_id: &'a str,
    pub properties: Json,
}

pub struct NotionApiClientParameters {
    pub base_url_override: Option<String>,
    pub api_key: String,
}

pub struct QueryDatabaseParameters<'a> {
    pub database_id: &'a str,
    pub start_cursor: Option<&'a str>,
    pub page_size: Option<NonZeroU32>,
    pub filter: Option<Json>,
}

#[derive(Default)]
pub struct RetryParameters<F> {
    pub custom_sleep: Option<F>,
}

pub struct UpdateDatabaseEntryParameters<'a> {
    pub entry_id: &'a str,
    pub properties: Json,
}
