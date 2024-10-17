use std::io::Result;
use std::{fs::OpenOptions, path::Path};

pub fn init(file_path: &Path) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    tracing_subscriber::fmt().json().with_writer(file).init();

    Ok(())
}
