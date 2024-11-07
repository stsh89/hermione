mod de;

use serde::Deserialize;
use serde_json::Value;
use std::{
    io::{Error, Result},
    str::FromStr,
};
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

pub fn get_database_properties(response: Response) -> Result<Vec<DatabaseProperty>> {
    let body: Value = response.into_json()?;
    let properties = body["properties"].as_object();

    let Some(properties) = properties else {
        return Err(Error::other("Missing Notion database properties"));
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
    response.into_json()
}

pub fn verify_commands_database_properties(properties: Vec<DatabaseProperty>) -> bool {
    let exptected_properties = commands_database_properties();

    verify_properties(exptected_properties, properties)
}

pub fn verify_workspaces_database_properties(properties: Vec<DatabaseProperty>) -> bool {
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
) -> bool {
    for property in expected_properties {
        if let Some(found) = properties.iter().find(|p| p.name == property.name) {
            if found.kind != property.kind {
                return false;
            }
        } else {
            return false;
        };
    }

    true
}

impl FromStr for DatabasePropertyKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let kind = match s {
            "created_time" => Self::CreatedTime,
            "rich_text" => Self::RichText,
            "title" => Self::Title,
            "last_edited_time" => Self::LastEditedTime,
            _ => {
                return Err(Error::other(format!(
                    "Can't convert `{}` into database property kind",
                    s
                )))
            }
        };

        Ok(kind)
    }
}
