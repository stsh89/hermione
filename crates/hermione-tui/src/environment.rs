use crate::Result;
use std::{
    io::Write,
    path::{Path, PathBuf},
};

pub struct Environment {
    app_path: PathBuf,
}

impl Environment {
    fn development_path() -> Result<PathBuf> {
        let dir = std::env::var("CARGO_MANIFEST_DIR")?;

        let mut buf = PathBuf::new();
        buf.push(dir);

        Ok(buf)
    }

    fn initialize(&self) -> Result<()> {
        let path = Path::new(&self.app_path);

        if !path.exists() {
            std::fs::create_dir(&self.app_path)?;
        }

        self.initialize_session()?;
        self.initialize_organizer()?;

        Ok(())
    }

    fn initialize_organizer(&self) -> Result<()> {
        let organizer_path = self.organizer_path()?;
        let path = Path::new(&organizer_path);

        if path.exists() {
            return Ok(());
        }

        let mut file = std::fs::File::create(path)?;
        file.write_all(b"[]")?;

        Ok(())
    }

    fn initialize_session(&self) -> Result<()> {
        let session_path = self.session_path()?;
        let path = Path::new(&session_path);

        if path.exists() {
            return Ok(());
        }

        std::fs::File::create(path)?;
        let mut file = std::fs::File::create(path)?;
        file.write_all(b"{}")?;

        Ok(())
    }

    fn new() -> Result<Self> {
        let mut app_path: PathBuf = match std::option_env!("HERMIONE_RELEASE") {
            Some(_) => Self::user_path()?,
            None => Self::development_path()?,
        };

        app_path.push(".hermione");

        Ok(Self { app_path })
    }

    pub fn organizer_path(&self) -> Result<String> {
        let path = Path::new(&self.app_path)
            .join("organizer.json")
            .into_os_string()
            .into_string()
            .map_err(|os_string| anyhow::anyhow!("can't convert os string: {os_string:?}"))?;

        Ok(path)
    }

    pub fn session_path(&self) -> Result<String> {
        let path = Path::new(&self.app_path)
            .join("session.json")
            .into_os_string()
            .into_string()
            .map_err(|os_string| anyhow::anyhow!("can't convert os string: {os_string:?}"))?;

        Ok(path)
    }

    pub fn setup() -> Result<Self> {
        let environment = Self::new()?;

        environment.initialize()?;

        Ok(environment)
    }

    fn user_path() -> Result<PathBuf> {
        let dir = dirs::home_dir().ok_or(anyhow::anyhow!("can't get home dir"))?;

        Ok(dir)
    }
}
