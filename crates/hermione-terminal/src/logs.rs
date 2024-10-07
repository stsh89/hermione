use crate::Result;
use std::{fs::OpenOptions, path::Path};

pub fn init(path: &Path) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.join("hermione.logs"))?;

    tracing_subscriber::fmt().json().with_writer(file).init();

    Ok(())
}
