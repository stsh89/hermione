use crate::{
    screen,
    settings::{NewSettingsParameters, Settings},
    Error, Result,
};
use std::path::{Path, PathBuf};

pub struct Command {
    file_path: PathBuf,
}

struct ReadOperation;

struct VerifyOperation(Settings);

struct WriteOperation(Settings);

impl Command {
    pub fn new(directory_path: &Path) -> Result<Self> {
        let file_path = Settings::file_path(directory_path);

        if file_path.try_exists()? {
            return Err(Error::msg("Settings file already exists"));
        }

        Ok(Self { file_path })
    }

    pub async fn execute(&self) -> Result<()> {
        ReadOperation::read()?
            .verify()
            .await?
            .write(&self.file_path)
    }
}

impl ReadOperation {
    fn read() -> Result<VerifyOperation> {
        screen::clear_and_reset_cursor();
        let api_key = screen::read_stdin("Enter your Notion API key: ")?;

        screen::clear_and_reset_cursor();
        let workspaces_page_id = screen::read_stdin("Enter your Notion workspaces page ID: ")?;

        let settings = Settings::new(NewSettingsParameters {
            api_key,
            workspaces_page_id,
        });

        Ok(VerifyOperation(settings))
    }
}

impl VerifyOperation {
    async fn verify(self) -> Result<WriteOperation> {
        screen::clear_and_reset_cursor();
        screen::print("Settings verification started...");

        self.0.verify().await?;

        screen::print("Settings verified successfully");

        Ok(WriteOperation(self.0))
    }
}

impl WriteOperation {
    fn write(self, file_path: &Path) -> Result<()> {
        self.0.write(file_path)?;

        screen::print(&format!("Settings file created: {}", file_path.display()));

        Ok(())
    }
}
