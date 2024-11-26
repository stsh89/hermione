mod de;

use eyre::{eyre, Report, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fmt::Display, str::FromStr};
use ureq::{serde::de::DeserializeOwned, Response};

pub struct DatabaseProperty {
    pub name: String,
    pub kind: DatabasePropertyKind,
}

#[derive(PartialEq)]
pub enum DatabasePropertyKind {
    Title,
    RichText,
    CreatedTime,
    LastEditedTime,
}

#[derive(Deserialize)]
pub struct NotionCommandProperties {
    #[serde(
        rename(deserialize = "Name"),
        deserialize_with = "de::title::deserializer"
    )]
    pub name: String,

    #[serde(
        rename(deserialize = "External ID"),
        deserialize_with = "de::rich_text::deserializer"
    )]
    pub external_id: String,

    #[serde(
        rename(deserialize = "Workspace ID"),
        deserialize_with = "de::rich_text::deserializer"
    )]
    pub workspace_id: String,

    #[serde(
        rename(deserialize = "Program"),
        deserialize_with = "de::rich_text::deserializer"
    )]
    pub program: String,
}

#[derive(Deserialize)]
pub struct NotionWorkspaceProperties {
    #[serde(
        rename(deserialize = "Name"),
        deserialize_with = "de::title::deserializer"
    )]
    pub name: String,

    #[serde(
        rename(deserialize = "External ID"),
        deserialize_with = "de::rich_text::deserializer"
    )]
    pub external_id: String,

    #[serde(
        rename(deserialize = "Location"),
        deserialize_with = "de::rich_text::deserializer"
    )]
    pub location: String,
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

#[derive(Serialize)]
struct RichTextFilter<'a> {
    property: String,
    rich_text: RichTextEqualsFilter<'a>,
}

#[derive(Serialize)]
struct RichTextEqualsFilter<'a> {
    equals: &'a str,
}

pub fn external_ids_filter(external_ids: Vec<String>) -> Option<Value> {
    if external_ids.is_empty() {
        return None;
    }

    let filters: Vec<RichTextFilter> = external_ids
        .iter()
        .map(|id| RichTextFilter {
            property: "External ID".to_string(),
            rich_text: RichTextEqualsFilter { equals: id },
        })
        .collect();

    Some(serde_json::json!({
        "or": serde_json::json!(filters),
    }))
}

pub fn get_database_properties(response: Response) -> Result<Vec<DatabaseProperty>> {
    let body: Value = response.into_json()?;
    let properties = body["properties"].as_object();

    let Some(properties) = properties else {
        return Err(eyre!(
            "Unexpected Notion response body. Missing field: properties"
        ));
    };

    let properties = properties
        .into_iter()
        .map(|(name, values)| {
            Ok(DatabaseProperty {
                name: name.to_string(),
                kind: values["type"].as_str().unwrap_or_default().parse()?,
            })
        })
        .collect::<Result<Vec<DatabaseProperty>>>()?;

    Ok(properties)
}

pub fn query_datrabase_response<T>(response: Response) -> Result<QueryDatabaseResponse<T>>
where
    T: DeserializeOwned,
{
    response.into_json().map_err(Report::new)
}

pub fn verify_commands_database_properties(properties: Vec<DatabaseProperty>) -> Result<()> {
    let exptected_properties = commands_database_properties();

    verify_properties(exptected_properties, properties)
}

pub fn verify_workspaces_database_properties(properties: Vec<DatabaseProperty>) -> Result<()> {
    let exptected_properties = workspaces_database_properties();

    verify_properties(exptected_properties, properties)
}

fn commands_database_properties() -> Vec<DatabaseProperty> {
    vec![
        DatabaseProperty {
            name: "External ID".into(),
            kind: DatabasePropertyKind::RichText,
        },
        DatabaseProperty {
            name: "Name".into(),
            kind: DatabasePropertyKind::Title,
        },
        DatabaseProperty {
            name: "Program".into(),
            kind: DatabasePropertyKind::RichText,
        },
        DatabaseProperty {
            name: "Workspace ID".into(),
            kind: DatabasePropertyKind::RichText,
        },
    ]
}

fn workspaces_database_properties() -> Vec<DatabaseProperty> {
    vec![
        DatabaseProperty {
            name: "External ID".into(),
            kind: DatabasePropertyKind::RichText,
        },
        DatabaseProperty {
            name: "Name".into(),
            kind: DatabasePropertyKind::Title,
        },
        DatabaseProperty {
            name: "Location".into(),
            kind: DatabasePropertyKind::RichText,
        },
    ]
}

fn verify_properties(
    expected_properties: Vec<DatabaseProperty>,
    properties: Vec<DatabaseProperty>,
) -> Result<()> {
    for property in expected_properties {
        if let Some(found) = properties.iter().find(|p| p.name == property.name) {
            if found.kind != property.kind {
                return Err(eyre!(
                    "Expected property {} to be of kind: {}, found: {}",
                    property.name,
                    property.kind,
                    found.kind
                ));
            }
        } else {
            return Err(eyre!("Missing property: {}", property.name));
        };
    }

    Ok(())
}

impl FromStr for DatabasePropertyKind {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let kind = match s {
            "created_time" => Self::CreatedTime,
            "rich_text" => Self::RichText,
            "title" => Self::Title,
            "last_edited_time" => Self::LastEditedTime,
            _ => return Err(eyre!("Unexpected database property kind: {}", s)),
        };

        Ok(kind)
    }
}

impl Display for DatabasePropertyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::CreatedTime => "created_time",
            Self::RichText => "rich_text",
            Self::Title => "title",
            Self::LastEditedTime => "last_edited_time",
        })
    }
}
