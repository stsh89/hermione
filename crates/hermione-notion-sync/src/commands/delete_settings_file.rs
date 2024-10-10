use crate::{settings::Settings, Result};
use std::{fs, path::PathBuf};

pub struct Command {
    directory_path: PathBuf,
}

impl Command {
    pub fn new(directory_path: PathBuf) -> Self {
        Self { directory_path }
    }

    pub fn execute(&self) -> Result<()> {
        let file_path = Settings::file_path(&self.directory_path);

        if !file_path.try_exists()? {
            return Ok(());
        }

        fs::remove_file(file_path)?;

        Ok(())
    }
}
