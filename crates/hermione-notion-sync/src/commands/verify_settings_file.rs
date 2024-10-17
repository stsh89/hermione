use crate::{screen, settings::Settings, Result};
use std::path::PathBuf;

pub struct Command {
    directory_path: PathBuf,
}

impl Command {
    pub fn new(directory_path: PathBuf) -> Self {
        Self { directory_path }
    }

    pub async fn execute(&self) -> Result<()> {
        screen::print("Settings verification started...");

        Settings::read(&self.directory_path)?.verify().await?;

        screen::print("Settings verified successfully");

        Ok(())
    }
}
