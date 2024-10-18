use crate::{settings::Settings, Result};
use std::{fs, path::PathBuf};

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

    async fn run(&self) -> Result<()> {
        let file_path = Settings::file_path(&self.directory_path);

        if !file_path.try_exists()? {
            return Ok(());
        }

        fs::remove_file(file_path)?;

        Ok(())
    }
}
