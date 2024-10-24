use hermione_ops::{
    notion::{
        Credentials, CredentialsParameters, DeleteCredentials, GetCredentials, SaveCredentials,
    },
    Result,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

const NOTION_CREDENTIALS_FILE_NAME: &str = "notion-sync.json";
const DATABASE_FILE_NAME: &str = "hermione.db3";

#[derive(Serialize, Deserialize)]
struct CredentialsJson {
    api_key: String,
    commands_page_id: String,
    workspaces_page_id: String,
}

pub struct FileSystemProvider<'a> {
    /// Directory where files are stored
    location: &'a Path,
}

impl<'a> FileSystemProvider<'a> {
    pub fn new(location: &'a Path) -> Self {
        Self { location }
    }

    fn notion_credentials_file_path(&self) -> PathBuf {
        self.location.join(NOTION_CREDENTIALS_FILE_NAME)
    }

    pub fn database_file_path(&self) -> PathBuf {
        self.location.join(DATABASE_FILE_NAME)
    }
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

impl DeleteCredentials for FileSystemProvider<'_> {
    fn delete(&self) -> Result<()> {
        let file_path = self.notion_credentials_file_path();

        if !file_path.try_exists().map_err(eyre::Error::new)? {
            return Err(eyre::eyre!("Notion credentials file doesn't exist").into());
        }

        std::fs::remove_file(file_path).map_err(eyre::Error::new)?;

        Ok(())
    }
}

impl GetCredentials for FileSystemProvider<'_> {
    fn get(&self) -> Result<Credentials> {
        let file_path = self.notion_credentials_file_path();

        if !file_path.try_exists()? {
            return Err(eyre::eyre!("Notion credentials file doesn't exist").into());
        }

        let file = File::open(&file_path)?;
        let credentials: CredentialsJson =
            serde_json::from_reader(file).map_err(eyre::Error::new)?;

        Ok(credentials.into())
    }
}

impl SaveCredentials for FileSystemProvider<'_> {
    fn save(&self, credentials: Credentials) -> Result<()> {
        let file = File::create(self.notion_credentials_file_path())?;
        let credentials: CredentialsJson = credentials.into();

        serde_json::to_writer_pretty(file, &credentials).map_err(eyre::Error::new)?;

        Ok(())
    }
}
