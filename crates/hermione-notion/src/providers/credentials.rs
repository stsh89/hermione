use crate::clients::file_system::{CredentialsFileData, FileSystemClient};
use hermione_ops::{
    notion::{
        Credentials, CredentialsParameters, DeleteCredentials, GetCredentials, SaveCredentials,
    },
    Result,
};

pub struct NotionCredentialsProvider {
    pub client: FileSystemClient,
}

impl From<CredentialsFileData> for Credentials {
    fn from(credentials: CredentialsFileData) -> Self {
        let CredentialsFileData {
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

impl From<Credentials> for CredentialsFileData {
    fn from(value: Credentials) -> Self {
        CredentialsFileData {
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
    fn get_credentials(&self) -> Result<Credentials> {
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
