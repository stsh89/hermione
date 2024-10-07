use crate::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct AppDir {
    app_path: PathBuf,
}

impl AppDir {
    pub fn new() -> Result<Self> {
        let is_release = cfg!(not(debug_assertions));

        let mut app_path = if is_release {
            user_path()?
        } else {
            development_path()?
        };

        app_path.push(".hermione");

        if !app_path.try_exists()? {
            fs::create_dir(&app_path)?;
        }

        Ok(Self { app_path })
    }

    pub fn path(&self) -> &Path {
        self.app_path.as_path()
    }
}

fn development_path() -> Result<PathBuf> {
    let dir = std::env::var("CARGO_MANIFEST_DIR")?;

    let mut buf = PathBuf::new();
    buf.push(dir);

    Ok(buf)
}

fn user_path() -> Result<PathBuf> {
    let dir = dirs::home_dir().ok_or(anyhow::anyhow!("Can't get user's home dir"))?;

    Ok(dir)
}
