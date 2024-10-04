mod base;

pub mod notion;
pub mod notion_serde;
pub mod powershell;

type Error = eyre::Error;
type Result<T> = eyre::Result<T>;
