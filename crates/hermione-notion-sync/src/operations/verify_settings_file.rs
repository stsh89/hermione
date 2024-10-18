use crate::{screen, settings::Settings, Result};
use std::path::PathBuf;

pub struct Operation {
    directory_path: PathBuf,
}

impl crate::Operation for Operation {
    fn execute(&self) -> crate::OperationResult {
        Box::pin(self.run())
    }
}

impl Operation {
    pub fn new(directory_path: PathBuf) -> Self {
        Self { directory_path }
    }

    pub async fn run(&self) -> Result<()> {
        screen::print("Settings verification started...");

        Settings::read(&self.directory_path)?.verify().await?;

        screen::print("Settings verified successfully");

        Ok(())
    }
}
