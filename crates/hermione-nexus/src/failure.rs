#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Backup failure: {0}")]
    Backup(String),

    #[error("Clipboard failure: {0}")]
    Clipboard(#[source] eyre::Error),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("{0} not found")]
    NotFound(String),

    #[error("Storage failure: {0}")]
    Storage(#[source] eyre::Error),

    #[error("System failure: {0}")]
    System(#[source] eyre::Error),

    #[error("{0} verification failed")]
    Verification(String),
}

#[cfg(test)]
mod tests {
    use super::Error;
    use uuid::Uuid;

    #[test]
    fn test_not_found() {
        let err = Error::NotFound(format!("Command {}", Uuid::nil().braced()));

        assert_eq!(
            err.to_string(),
            "Command {00000000-0000-0000-0000-000000000000} not found"
        );
    }
}
