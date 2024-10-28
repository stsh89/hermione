use crate::clients::standard_data_stream::{CredentialsStream, StandardDataStreamClient};
use hermione_ops::{
    notion::{Credentials, CredentialsParameters, GetCredentials},
    Result,
};

pub struct ScreenProvider {
    client: StandardDataStreamClient,
}

impl ScreenProvider {
    fn enter_credentials(&self) -> Result<Credentials> {
        let credentials = self.client.read_credentials()?;

        Ok(credentials.into())
    }

    pub fn new() -> Self {
        Self {
            client: StandardDataStreamClient::new(),
        }
    }

    pub fn show_credentials(&self, credentials: Credentials) -> Result<()> {
        self.client.write_credentials(credentials.into())?;

        Ok(())
    }
}

impl GetCredentials for ScreenProvider {
    fn get_credentials(&self) -> Result<Credentials> {
        self.enter_credentials()
    }
}

impl From<Credentials> for CredentialsStream {
    fn from(value: Credentials) -> Self {
        Self {
            api_key: value.api_key().into(),
            commands_page_id: value.commands_page_id().into(),
            workspaces_page_id: value.workspaces_page_id().into(),
        }
    }
}

impl From<CredentialsStream> for Credentials {
    fn from(value: CredentialsStream) -> Self {
        let CredentialsStream {
            api_key,
            commands_page_id,
            workspaces_page_id,
        } = value;

        Self::new(CredentialsParameters {
            api_key,
            commands_page_id,
            workspaces_page_id,
        })
    }
}
