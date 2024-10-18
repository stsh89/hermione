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
        let settings = Settings::read(&self.directory_path)?;
        let file_content = serde_json::to_string_pretty(&settings)?;

        screen::print(&file_content);

        Ok(())
    }
}
