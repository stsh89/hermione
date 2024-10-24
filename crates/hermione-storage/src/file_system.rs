use eyre::{Error, Result};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub const TERMINAL_APP_LOGS_FILE_NAME_PREFIX: &str = "hermione-terminal-logs";

pub struct FileSystemProvider {
    /// Terminal app folder path
    location: PathBuf,
}

impl FileSystemProvider {
    pub fn new() -> Result<Self> {
        let location = path()?;

        Ok(Self { location })
    }

    #[cfg(feature = "notion")]
    pub fn notion_credentials_file_path(&self) -> PathBuf {
        self.location.join("notion.json")
    }

    pub fn database_file_path(&self) -> PathBuf {
        self.location.join("hermione.db3")
    }

    pub fn location(&self) -> &Path {
        self.location.as_path()
    }
}

pub fn path() -> Result<PathBuf> {
    let is_release = cfg!(not(debug_assertions));

    let mut app_path = if is_release {
        user_path()?
    } else {
        development_path()?
    };

    app_path.push(".hermione");

    if !app_path.try_exists()? {
        std::fs::create_dir(&app_path)?;
    }

    Ok(app_path.to_path_buf())
}

fn development_path() -> Result<PathBuf> {
    let output = Command::new("cargo")
        .args(["locate-project", "--workspace", "--message-format", "plain"])
        .output()?;

    Path::new(std::str::from_utf8(&output.stdout)?)
        .parent()
        .map(|path| path.to_path_buf())
        .ok_or(Error::msg("Missing terminal app development path"))
}

fn user_path() -> Result<PathBuf> {
    let dir = dirs::home_dir().ok_or(Error::msg("Can't get user's home dir"))?;

    Ok(dir)
}
