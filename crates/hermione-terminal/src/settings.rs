use crate::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    path::Path,
};

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub storage: Storage,
}

#[derive(Deserialize, Serialize)]
pub enum Storage {
    #[serde(rename = "json")]
    Json,

    #[serde(rename = "notion")]
    Notion(NotionStorageSettings),
}

#[derive(Deserialize, Serialize)]
pub struct NotionStorageSettings {}

impl Settings {
    pub fn new(app_path: &Path) -> Result<Self> {
        let path = app_path.join("settings.json");

        if !path.try_exists()? {
            let settings = Settings {
                storage: Storage::Json,
            };

            settings.save(&path)?;
            return Ok(settings);
        }

        let file = File::open(&path)?;
        let settings = serde_json::from_reader(file)?;

        Ok(settings)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;

        serde_json::to_writer(&mut file, &self)?;

        Ok(())
    }
}
