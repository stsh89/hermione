use crate::Result;
use std::fs::OpenOptions;

pub fn init(path: &str) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    tracing_subscriber::fmt().json().with_writer(file).init();

    Ok(())
}
