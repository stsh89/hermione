use crate::{Error, Result};
use std::{future::Future, ops::Deref, str::FromStr};

pub trait DeleteCredentials {
    fn delete(&self) -> Result<()>;
}

pub trait GetCredentials {
    fn get_credentials(&self) -> Result<Credentials>;
}

pub trait GetDatabaseProperties {
    fn get_database_properties(
        &self,
        api_key: &ApiKey,
        database_id: &DatabaseId,
    ) -> impl Future<Output = Result<Vec<DatabaseProperty>>>;
}

pub trait SaveCredentials {
    fn save(&self, credentials: Credentials) -> Result<()>;
}

pub struct DeleteCredentialsOperation<'a, T> {
    pub deleter: &'a T,
}

pub struct GetCredentialsOperation<'a, T> {
    pub get_credentials_provider: &'a T,
}

pub struct SaveCredentialsOperation<'a, SCP, GCP, GPPP> {
    pub get_credentials_provider: &'a GCP,
    pub get_database_properties_provider: &'a GPPP,
    pub save_credentials_provider: &'a SCP,
}

pub struct VerifyCredentialsOperation<'a, GPPP> {
    pub get_database_properties_provider: &'a GPPP,
}

pub struct Credentials {
    api_key: ApiKey,
    commands_database_id: DatabaseId,
    workspaces_database_id: DatabaseId,
}

pub struct ApiKey {
    value: String,
}

pub struct DatabaseId {
    value: String,
}

pub struct CredentialsParameters {
    pub api_key: String,
    pub commands_database_id: String,
    pub workspaces_database_id: String,
}

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

impl<'a, T> DeleteCredentialsOperation<'a, T>
where
    T: DeleteCredentials,
{
    pub fn execute(&self) -> Result<()> {
        tracing::info!(operation = "Delete Notion credentials");

        self.deleter.delete()
    }
}

impl<'a, T> GetCredentialsOperation<'a, T>
where
    T: GetCredentials,
{
    pub fn execute(&self) -> Result<Credentials> {
        tracing::info!(operation = "Get Notion credentials");

        self.get_credentials_provider.get_credentials()
    }
}

impl<'a, SCP, GCP, GPPP> SaveCredentialsOperation<'a, SCP, GCP, GPPP>
where
    GCP: GetCredentials,
    SCP: SaveCredentials,
    GPPP: GetDatabaseProperties,
{
    pub async fn execute(&self) -> Result<()> {
        tracing::info!(operation = "Save Notion credentials");

        let credentials = GetCredentialsOperation {
            get_credentials_provider: self.get_credentials_provider,
        }
        .execute()?;

        VerifyCredentialsOperation {
            get_database_properties_provider: self.get_database_properties_provider,
        }
        .execute(&credentials)
        .await?;

        self.save_credentials_provider.save(credentials)
    }
}

impl<'a, GPPP> VerifyCredentialsOperation<'a, GPPP>
where
    GPPP: GetDatabaseProperties,
{
    pub async fn execute(&self, credentials: &Credentials) -> Result<()> {
        tracing::info!(operation = "Verify Notion credentials");

        self.verify_commands_database_properties(credentials)
            .await?;
        self.verify_workspaces_database_properties(credentials)
            .await?;

        Ok(())
    }

    async fn verify_commands_database_properties(&self, credentials: &Credentials) -> Result<()> {
        let database_properties = self
            .get_database_properties_provider
            .get_database_properties(credentials.api_key(), credentials.commands_database_id())
            .await?;

        let exptected_properties = commands_database_properties();

        if !verify_properties(exptected_properties, database_properties) {
            return Err(Error::FailedPrecondition(
                "Invalid Notion commands database properties".into(),
            ));
        }

        Ok(())
    }

    async fn verify_workspaces_database_properties(&self, credentials: &Credentials) -> Result<()> {
        let database_properties = self
            .get_database_properties_provider
            .get_database_properties(credentials.api_key(), credentials.workspaces_database_id())
            .await?;

        let exptected_properties = workspaces_database_properties();

        if !verify_properties(exptected_properties, database_properties) {
            return Err(Error::FailedPrecondition(
                "Invalid Notion workspaces database properties".into(),
            ));
        }

        Ok(())
    }
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

impl Credentials {
    pub fn api_key(&self) -> &ApiKey {
        &self.api_key
    }

    pub fn commands_database_id(&self) -> &DatabaseId {
        &self.commands_database_id
    }

    pub fn new(parameters: CredentialsParameters) -> Self {
        let CredentialsParameters {
            api_key,
            commands_database_id,
            workspaces_database_id,
        } = parameters;

        Self {
            api_key: ApiKey { value: api_key },
            commands_database_id: DatabaseId {
                value: commands_database_id,
            },
            workspaces_database_id: DatabaseId {
                value: workspaces_database_id,
            },
        }
    }

    pub fn workspaces_database_id(&self) -> &DatabaseId {
        &self.workspaces_database_id
    }
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

impl Deref for ApiKey {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.value.as_str()
    }
}

impl Deref for DatabaseId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.value.as_str()
    }
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
                return Err(Error::Internal(format!(
                    "Can't convert `{}` into database property kind",
                    s
                )))
            }
        };

        Ok(kind)
    }
}
