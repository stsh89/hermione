mod base;

pub mod notion;
pub mod notion_serde;
pub mod powershell;

type Result<T> = eyre::Result<T>;
