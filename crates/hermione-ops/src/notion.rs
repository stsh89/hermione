use crate::Result;
use std::future::Future;

pub trait DeleteCredentials {
    fn delete(&self) -> Result<()>;
}

pub trait GetCredentials {
    fn get(&self) -> Result<Credentials>;
}

pub trait SaveCredentials {
    fn save(&self, credentials: Credentials) -> Result<()>;
}

pub trait VerifyCredentials {
    fn verify(&self, credentials: &Credentials) -> impl Future<Output = Result<()>> + Send;
}

pub struct DeleteCredentialsOperation<'a, T> {
    pub deleter: &'a T,
}

pub struct GetCredentialsOperation<'a, T> {
    pub get_credentials_provider: &'a T,
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

impl<'a, T> GetCredentialsOperation<'a, T>
where
    T: GetCredentials,
{
    pub fn execute(&self) -> Result<Credentials> {
        self.get_credentials_provider.get()
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
