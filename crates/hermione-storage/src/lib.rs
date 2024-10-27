use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
    str,
};

#[cfg(feature = "extensions")]
mod extensions;

#[cfg(feature = "backup")]
pub mod backup;

pub mod sqlite;

/// File system location for the terminal app files
pub fn app_path() -> io::Result<PathBuf> {
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

fn development_path() -> io::Result<PathBuf> {
    let output = Command::new("cargo")
        .args(["locate-project", "--workspace", "--message-format", "plain"])
        .output()?;

    let project_path = str::from_utf8(&output.stdout)
        .map_err(|_err| io::Error::other("Can't read project path"))?;

    Path::new(project_path)
        .parent()
        .map(|path| path.to_path_buf())
        .ok_or(io::Error::other("Missing terminal app development path"))
}

fn user_path() -> io::Result<PathBuf> {
    let dir = dirs::home_dir().ok_or(io::Error::other("Can't get user's home dir"))?;

    Ok(dir)
}
