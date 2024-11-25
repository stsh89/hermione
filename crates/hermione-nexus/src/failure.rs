use uuid::Uuid;

use crate::definitions::BackupProviderKind;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Backup failure: {0}")]
    Backup(#[source] eyre::Error),

    #[error("Could not find {0} backup credentials")]
    BackupCredentialsNotFound(BackupProviderKind),

    #[error("Could not find command with ID: {0}")]
    CommandNotFound(Uuid),

    #[error("Failed to verify backup credentials: {0}")]
    BackupCredentialsVerification(#[source] eyre::Error),

    #[error("Clipboard failure: {0}")]
    Clipboard(#[source] eyre::Error),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Storage failure: {0}")]
    Storage(#[source] eyre::Error),

    #[error("System failure: {0}")]
    System(#[source] eyre::Error),

    #[error("{0} verification failed")]
    Verification(String),

    #[error("Could not find workspace with ID: {0}")]
    WorkspaceNotFound(Uuid),
}

#[cfg(test)]
mod tests {
    use super::Error;
    use crate::definitions::BackupProviderKind;
    use uuid::Uuid;

    #[test]
    fn test_backup_credentials_not_found() {
        assert_eq!(
            Error::BackupCredentialsNotFound(BackupProviderKind::Notion).to_string(),
            "Could not find Notion backup credentials"
        );
    }

    #[test]
    fn test_command_not_found() {
        assert_eq!(
            Error::CommandNotFound(Uuid::nil()).to_string(),
            "Could not find command with ID: 00000000-0000-0000-0000-000000000000"
        );
    }

    #[test]
    fn test_workspace_not_found() {
        assert_eq!(
            Error::WorkspaceNotFound(Uuid::nil()).to_string(),
            "Could not find workspace with ID: 00000000-0000-0000-0000-000000000000"
        );
    }
}
