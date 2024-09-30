use crate::Result;
use std::path::{Path, PathBuf};

pub struct Settings {
    app_path: PathBuf,
}

impl Settings {
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

        Ok(())
    }

    fn new() -> Result<Self> {
        let is_release = cfg!(not(debug_assertions));

        let mut app_path = if is_release {
            Self::user_path()?
        } else {
            Self::development_path()?
        };

        app_path.push(".hermione");

        Ok(Self { app_path })
    }

    pub fn path_to_memories(&self) -> &PathBuf {
        &self.app_path
    }

    pub fn logs_path(&self) -> Result<String> {
        let path = Path::new(&self.app_path)
            .join("hermione.logs")
            .into_os_string()
            .into_string()
            .map_err(|os_string| anyhow::anyhow!("can't convert os string: {os_string:?}"))?;

        Ok(path)
    }

    pub fn setup() -> Result<Self> {
        let settings = Self::new()?;

        settings.initialize()?;

        Ok(settings)
    }

    fn user_path() -> Result<PathBuf> {
        let dir = dirs::home_dir().ok_or(anyhow::anyhow!("Can't get user's home dir"))?;

        Ok(dir)
    }
}
