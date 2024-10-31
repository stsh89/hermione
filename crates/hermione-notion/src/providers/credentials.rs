use std::{fs::File, io, path::PathBuf};

use hermione_ops::{
    notion::{
        Credentials, CredentialsParameters, DeleteCredentials, GetCredentials, SaveCredentials,
    },
    Result,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CredentialsFileData {
    pub api_key: String,
    pub commands_database_id: String,
    pub workspaces_database_id: String,
}

pub struct NotionCredentialsProvider {
    credentials_file_path: PathBuf,
}

impl NotionCredentialsProvider {
    fn credentials_file_path_try_exists(&self) -> io::Result<()> {
        let file_path = &self.credentials_file_path;

        if file_path.try_exists()? {
            return Ok(());
        }

        Err(io::Error::other("Notion credentials file doesn't exist"))
    }

    pub fn delete_credentials(&self) -> io::Result<()> {
        self.credentials_file_path_try_exists()?;

        std::fs::remove_file(&self.credentials_file_path)
    }

    pub fn new(credentials_file_path: PathBuf) -> Self {
        Self {
            credentials_file_path,
        }
    }

    pub fn read_credentials(&self) -> io::Result<CredentialsFileData> {
        self.credentials_file_path_try_exists()?;

        let file = File::open(&self.credentials_file_path)?;

        serde_json::from_reader(file)
            .map_err(|_err| io::Error::other("Failed to parse Notion's credentials file"))
    }

    pub fn write_credentials(&self, credentials: CredentialsFileData) -> io::Result<()> {
        let file = File::create(&self.credentials_file_path)?;

        serde_json::to_writer(file, &credentials)
            .map_err(|_err| io::Error::other("Failed to write Notion's credentials file"))
    }
}

impl From<CredentialsFileData> for Credentials {
    fn from(credentials: CredentialsFileData) -> Self {
        let CredentialsFileData {
            api_key,
            commands_database_id,
            workspaces_database_id,
        } = credentials;

        Self::new(CredentialsParameters {
            api_key,
            commands_database_id,
            workspaces_database_id,
        })
    }
}

impl From<Credentials> for CredentialsFileData {
    fn from(value: Credentials) -> Self {
        CredentialsFileData {
            api_key: value.api_key().to_string(),
            commands_database_id: value.commands_database_id().to_string(),
            workspaces_database_id: value.workspaces_database_id().to_string(),
        }
    }
}

impl DeleteCredentials for NotionCredentialsProvider {
    fn delete(&self) -> Result<()> {
        self.delete_credentials()?;

        Ok(())
    }
}

impl GetCredentials for NotionCredentialsProvider {
    fn get_credentials(&self) -> Result<Credentials> {
        let credentials = self.read_credentials()?;

        Ok(credentials.into())
    }
}

impl SaveCredentials for NotionCredentialsProvider {
    fn save(&self, credentials: Credentials) -> Result<()> {
        self.write_credentials(credentials.into())?;

        Ok(())
    }
}
