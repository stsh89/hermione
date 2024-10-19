use crate::{
    screen::{self, Input},
    settings::{NewSettingsParameters, Settings},
    Error, Result,
};
use std::path::{Path, PathBuf};

const API_KEY_PROMPT: &str = "Enter your Notion API key";
const COMMANDS_PAGE_ID_PROMPT: &str = "Enter your Notion commands page ID";
const WORKSPACES_PAGE_ID_PROMPT: &str = "Enter your Notion workspaces page ID";

pub struct Operation {
    file_path: PathBuf,
}

impl crate::Operation for Operation {
    fn execute(&self) -> crate::OperationResult {
        Box::pin(self.run())
    }
}

struct ReadOperation;

struct VerifyOperation(Settings);

struct WriteOperation(Settings);

impl Operation {
    pub fn new(directory_path: &Path) -> Result<Self> {
        let file_path = Settings::file_path(directory_path);

        if file_path.try_exists()? {
            return Err(Error::msg("Settings file already exists"));
        }

        Ok(Self { file_path })
    }

    async fn run(&self) -> Result<()> {
        ReadOperation::read()?
            .verify()
            .await?
            .write(&self.file_path)
    }
}

impl ReadOperation {
    fn read() -> Result<VerifyOperation> {
        let read_input = |prompt: &str| {
            screen::clear_and_reset_cursor();
            Input::new(prompt).read()
        };

        let settings = Settings::new(NewSettingsParameters {
            api_key: read_input(API_KEY_PROMPT)?,
            commands_page_id: read_input(COMMANDS_PAGE_ID_PROMPT)?,
            workspaces_page_id: read_input(WORKSPACES_PAGE_ID_PROMPT)?,
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
