use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Error, Result},
    path::PathBuf,
};

pub struct FileSystemClient {
    credentials_file_path: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct CredentialsFileData {
    pub api_key: String,
    pub commands_page_id: String,
    pub workspaces_page_id: String,
}

impl FileSystemClient {
    fn credentials_file_path_try_exists(&self) -> Result<()> {
        let file_path = &self.credentials_file_path;

        if file_path.try_exists()? {
            return Ok(());
        }

        Err(Error::other("Notion credentials file doesn't exist"))
    }

    pub fn delete_credentials(&self) -> Result<()> {
        self.credentials_file_path_try_exists()?;

        std::fs::remove_file(&self.credentials_file_path)
    }

    pub fn new(credentials_file_path: PathBuf) -> Self {
        Self {
            credentials_file_path,
        }
    }

    pub fn read_credentials(&self) -> Result<CredentialsFileData> {
        self.credentials_file_path_try_exists()?;

        let file = File::open(&self.credentials_file_path)?;

        serde_json::from_reader(file)
            .map_err(|_err| Error::other("Failed to parse Notion's credentials file"))
    }

    pub fn write_credentials(&self, credentials: CredentialsFileData) -> Result<()> {
        let file = File::create(&self.credentials_file_path)?;

        serde_json::to_writer(file, &credentials)
            .map_err(|_err| Error::other("Failed to write Notion's credentials file"))
    }
}
