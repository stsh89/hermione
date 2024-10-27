use crate::clients::credentials::{CredentialsJson, NotionCredentialsClient};
use hermione_ops::{
    notion::{
        Credentials, CredentialsParameters, DeleteCredentials, GetCredentials, SaveCredentials,
    },
    Result,
};

pub struct NotionCredentialsProvider {
    pub client: NotionCredentialsClient,
}

impl From<CredentialsJson> for Credentials {
    fn from(credentials: CredentialsJson) -> Self {
        let CredentialsJson {
            api_key,
            commands_page_id,
            workspaces_page_id,
        } = credentials;

        Self::new(CredentialsParameters {
            api_key,
            commands_page_id,
            workspaces_page_id,
        })
    }
}

impl From<Credentials> for CredentialsJson {
    fn from(value: Credentials) -> Self {
        CredentialsJson {
            api_key: value.api_key().into(),
            commands_page_id: value.commands_page_id().into(),
            workspaces_page_id: value.workspaces_page_id().into(),
        }
    }
}

impl DeleteCredentials for NotionCredentialsProvider {
    fn delete(&self) -> Result<()> {
        self.client.delete_credentials()?;

        Ok(())
    }
}

impl GetCredentials for NotionCredentialsProvider {
    fn get(&self) -> Result<Credentials> {
        let credentials = self.client.read_credentials()?;

        Ok(credentials.into())
    }
}

impl SaveCredentials for NotionCredentialsProvider {
    fn save(&self, credentials: Credentials) -> Result<()> {
        self.client.write_credentials(credentials.into())?;

        Ok(())
    }
}
