use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

type Result<T> = anyhow::Result<T>;

pub fn path() -> Result<PathBuf> {
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

    Ok(app_path.to_path_buf())
}

fn development_path() -> Result<PathBuf> {
    let output = Command::new("cargo")
        .args(["locate-project", "--workspace", "--message-format", "plain"])
        .output()?;

    Path::new(std::str::from_utf8(&output.stdout)?)
        .parent()
        .map(|path| path.to_path_buf())
        .ok_or(anyhow::anyhow!("Can't get project location"))
}

fn user_path() -> Result<PathBuf> {
    let dir = dirs::home_dir().ok_or(anyhow::anyhow!("Can't get user's home dir"))?;

    Ok(dir)
}
