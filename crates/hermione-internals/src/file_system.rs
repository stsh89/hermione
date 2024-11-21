use crate::{ApplicationState, APPLICATION_STATE};
use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

pub type AppLocationResult<T> = Result<T, AppLocationError>;

#[derive(Debug, thiserror::Error)]
pub enum AppLocationError {
    #[error(transparent)]
    IO(#[from] io::Error),

    #[error("Failed to locate project development directory")]
    MissingProjectDevelopmentDirectory,

    #[error("Failed to locate user home directory")]
    MissingUserHomeDirectory,

    #[error("Unexpected error: {0}")]
    Unexpected(&'static str),
}

pub struct AppLocation {
    directory: PathBuf,
}

impl AppLocation {
    pub fn directory(&self) -> &Path {
        &self.directory
    }

    pub fn locate() -> AppLocationResult<Self> {
        let mut directory = match APPLICATION_STATE {
            ApplicationState::Release => user_path()?,
            ApplicationState::Development => development_path()?,
        };

        directory.push(".hermione");

        if !directory.try_exists()? {
            fs::create_dir(&directory)?;
        }

        Ok(Self { directory })
    }
}

fn development_path() -> AppLocationResult<PathBuf> {
    let output = Command::new("cargo")
        .args(["locate-project", "--workspace", "--message-format", "plain"])
        .output()?;

    let project_path = std::str::from_utf8(&output.stdout).map_err(|_err| {
        AppLocationError::Unexpected("Project development path contains non UTF-8 symbols")
    })?;

    Path::new(project_path)
        .parent()
        .map(|path| path.to_path_buf())
        .ok_or(AppLocationError::MissingProjectDevelopmentDirectory)
}

fn user_path() -> AppLocationResult<PathBuf> {
    dirs::home_dir().ok_or(AppLocationError::MissingUserHomeDirectory)
}
