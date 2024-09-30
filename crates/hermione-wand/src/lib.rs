pub mod powershell;

type Error = anyhow::Error;
type Result<T> = std::result::Result<T, Error>;
