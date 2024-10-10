use crate::{Error, Result};
use hermione_notion::{Client, NewClientParameters, QueryDatabaseParameters};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

const SETTINGS_FILE_PATH: &str = "notion-sync.json";

#[derive(Serialize, Deserialize)]
pub struct Settings {
    api_key: String,
    commands_page_id: String,
    workspaces_page_id: String,
}

pub struct NewSettingsParameters {
    pub api_key: String,
    pub commands_page_id: String,
    pub workspaces_page_id: String,
}

impl Settings {
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn commands_page_id(&self) -> &str {
        &self.commands_page_id
    }

    pub fn file_path(directory_path: &Path) -> PathBuf {
        directory_path.join(SETTINGS_FILE_PATH)
    }

    pub fn new(parameters: NewSettingsParameters) -> Self {
        let NewSettingsParameters {
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

    pub fn read(directory_path: &Path) -> Result<Self> {
        let file_path = Settings::file_path(directory_path);

        if !file_path.try_exists()? {
            return Err(Error::msg("Settings file not found"));
        }

        let file = File::open(file_path)?;
        let settings: Self = serde_json::from_reader(file)?;

        Ok(settings)
    }

    pub async fn verify(&self) -> Result<()> {
        let client = Client::new(NewClientParameters {
            api_key: Some(self.api_key().into()),
            ..Default::default()
        })?;

        client
            .query_database(
                self.workspaces_page_id(),
                QueryDatabaseParameters {
                    page_size: 1,
                    ..Default::default()
                },
            )
            .await?;

        client
            .query_database(
                self.commands_page_id(),
                QueryDatabaseParameters {
                    page_size: 1,
                    ..Default::default()
                },
            )
            .await?;

        Ok(())
    }

    pub fn workspaces_page_id(&self) -> &str {
        &self.workspaces_page_id
    }

    pub fn write(&self, file_path: &Path) -> Result<()> {
        let file = File::create(file_path)?;
        serde_json::to_writer_pretty(file, self)?;

        Ok(())
    }
}
