#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Internal: {0}")]
    Internal(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("{0} not found")]
    NotFound(String),

    #[error(transparent)]
    Storage(#[from] eyre::Error),
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::Error;

    #[test]
    fn test_not_found() {
        let err = Error::NotFound(format!("Command {}", Uuid::nil().braced()));

        assert_eq!(
            err.to_string(),
            "Command {00000000-0000-0000-0000-000000000000} not found"
        );
    }
}
