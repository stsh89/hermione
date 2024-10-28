use crate::{
    backup::{BackupOperation, Import, Iterate, ListByIds, Update},
    commands::Command,
    workspaces::Workspace,
    Result,
};
use std::future::Future;

pub trait DeleteCredentials {
    fn delete(&self) -> Result<()>;
}

pub trait GetCredentials {
    fn get_credentials(&self) -> Result<Credentials>;
}

pub trait SaveCredentials {
    fn save(&self, credentials: Credentials) -> Result<()>;
}

pub trait VerifyCredentials {
    fn verify(&self, credentials: &Credentials) -> impl Future<Output = Result<()>>;
}

pub struct DeleteCredentialsOperation<'a, T> {
    pub deleter: &'a T,
}

pub struct ExportOperation<'a, LCP, NCP, LWP, NWP> {
    pub local_commands_provider: &'a LCP,
    pub notion_commands_provider: &'a NCP,
    pub local_workspaces_provider: &'a LWP,
    pub notion_workspaces_provider: &'a NWP,
}

pub struct GetCredentialsOperation<'a, T> {
    pub get_credentials_provider: &'a T,
}

pub struct ImportOperation<'a, LCP, NCP, LWP, NWP> {
    pub local_commands_provider: &'a LCP,
    pub notion_commands_provider: &'a NCP,
    pub local_workspaces_provider: &'a LWP,
    pub notion_workspaces_provider: &'a NWP,
}

pub struct SaveCredentialsOperation<'a, S, G, V> {
    pub saver: &'a S,
    pub getter: &'a G,
    pub verifier: &'a V,
}

pub struct VerifyCredentialsOperation<'a, G, V> {
    pub get_credentials_provider: &'a G,
    pub verify_credentials_provider: &'a V,
}

pub struct Credentials {
    api_key: String,
    commands_page_id: String,
    workspaces_page_id: String,
}

pub struct CredentialsParameters {
    pub api_key: String,
    pub commands_page_id: String,
    pub workspaces_page_id: String,
}

impl<'a, T> DeleteCredentialsOperation<'a, T>
where
    T: DeleteCredentials,
{
    pub fn execute(&self) -> Result<()> {
        self.deleter.delete()
    }
}

impl<'a, LCP, NCP, LWP, NWP> ExportOperation<'a, LCP, NCP, LWP, NWP>
where
    LCP: Iterate<Entity = Command>,
    NCP: Import<Entity = Command> + ListByIds<Entity = Command> + Update<Entity = Command>,
    LWP: Iterate<Entity = Workspace>,
    NWP: Import<Entity = Workspace> + ListByIds<Entity = Workspace> + Update<Entity = Workspace>,
{
    pub async fn execute(&self) -> Result<()> {
        BackupOperation {
            local_provider: self.local_workspaces_provider,
            remote_provider: self.notion_workspaces_provider,
        }
        .execute()
        .await?;

        BackupOperation {
            local_provider: self.local_commands_provider,
            remote_provider: self.notion_commands_provider,
        }
        .execute()
        .await?;

        Ok(())
    }
}

impl<'a, T> GetCredentialsOperation<'a, T>
where
    T: GetCredentials,
{
    pub fn execute(&self) -> Result<Credentials> {
        self.get_credentials_provider.get_credentials()
    }
}

impl<'a, LCP, NCP, LWP, NWP> ImportOperation<'a, LCP, NCP, LWP, NWP>
where
    NCP: Iterate<Entity = Command>,
    LCP: Import<Entity = Command> + ListByIds<Entity = Command> + Update<Entity = Command>,
    NWP: Iterate<Entity = Workspace>,
    LWP: Import<Entity = Workspace> + ListByIds<Entity = Workspace> + Update<Entity = Workspace>,
{
    pub async fn execute(&self) -> Result<()> {
        BackupOperation {
            local_provider: self.notion_workspaces_provider,
            remote_provider: self.local_workspaces_provider,
        }
        .execute()
        .await?;

        BackupOperation {
            local_provider: self.notion_commands_provider,
            remote_provider: self.local_commands_provider,
        }
        .execute()
        .await?;

        Ok(())
    }
}

impl<'a, S, G, V> SaveCredentialsOperation<'a, S, G, V>
where
    G: GetCredentials,
    S: SaveCredentials,
    V: VerifyCredentials,
{
    pub async fn execute(&self) -> Result<()> {
        let credentials = GetCredentialsOperation {
            get_credentials_provider: self.getter,
        }
        .execute()?;

        self.verifier.verify(&credentials).await?;
        self.saver.save(credentials)
    }
}

impl<'a, G, V> VerifyCredentialsOperation<'a, G, V>
where
    G: GetCredentials,
    V: VerifyCredentials,
{
    pub async fn execute(&self) -> Result<()> {
        let credentials = GetCredentialsOperation {
            get_credentials_provider: self.get_credentials_provider,
        }
        .execute()?;

        self.verify_credentials_provider.verify(&credentials).await
    }
}

impl Credentials {
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn commands_page_id(&self) -> &str {
        &self.commands_page_id
    }

    pub fn new(parameters: CredentialsParameters) -> Self {
        let CredentialsParameters {
            api_key,
            commands_page_id,
            workspaces_page_id,
        } = parameters;

        Self {
            api_key,
            commands_page_id,
            workspaces_page_id,
        }
    }

    pub fn workspaces_page_id(&self) -> &str {
        &self.workspaces_page_id
    }
}
