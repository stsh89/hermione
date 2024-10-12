use crate::{screen, settings::Settings, Result};
use std::path::PathBuf;

pub struct Command {
    directory_path: PathBuf,
}

impl Command {
    pub fn new(directory_path: PathBuf) -> Self {
        Self { directory_path }
    }

    pub fn execute(&self) -> Result<()> {
        let settings = Settings::read(&self.directory_path)?;
        let file_content = serde_json::to_string_pretty(&settings)?;

        screen::print(&file_content);

        Ok(())
    }
}
