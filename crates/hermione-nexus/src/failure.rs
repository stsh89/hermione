use eyre::Report;

#[derive(Debug, thiserror::Error)]
#[error("{source}")]
pub struct Error {
    kind: ErrorKind,
    source: Report,
}

#[derive(Debug)]
enum ErrorKind {
    BackupServiceComunication,
    BackupServiceConfiguration,
    InvalidArgument,
    BackupServiceDataCorruption,
    NotFound,
    Storage,
    System,
    Unexpected,
}

impl Error {
    pub fn backup_service_communication(source: Report) -> Self {
        Self {
            kind: ErrorKind::BackupServiceComunication,
            source,
        }
    }

    pub fn backup_service_configuration(source: Report) -> Self {
        Self {
            kind: ErrorKind::BackupServiceConfiguration,
            source,
        }
    }

    pub fn backup_service_data_corruption(source: Report) -> Self {
        Self {
            kind: ErrorKind::BackupServiceDataCorruption,
            source,
        }
    }

    pub fn is_invalid_argument(&self) -> bool {
        matches!(self.kind, ErrorKind::InvalidArgument)
    }

    pub fn is_invalid_backup_data(&self) -> bool {
        matches!(self.kind, ErrorKind::BackupServiceDataCorruption)
    }

    pub fn is_not_found(&self) -> bool {
        matches!(self.kind, ErrorKind::NotFound)
    }

    pub fn is_storage(&self) -> bool {
        matches!(self.kind, ErrorKind::Storage)
    }

    pub fn invalid_argument(source: Report) -> Self {
        Self {
            kind: ErrorKind::InvalidArgument,
            source,
        }
    }

    pub fn not_found(source: Report) -> Self {
        Self {
            kind: ErrorKind::NotFound,
            source,
        }
    }

    pub fn storage(source: Report) -> Self {
        Self {
            kind: ErrorKind::Storage,
            source,
        }
    }

    pub fn system(source: Report) -> Self {
        Self {
            kind: ErrorKind::System,
            source,
        }
    }

    pub fn unexpected(source: Report) -> Self {
        Self {
            kind: ErrorKind::Unexpected,
            source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::eyre;
    use uuid::Uuid;

    #[test]
    fn invalid_argument() {
        let err = Error::invalid_argument(eyre!("Workspace ID cannot be nil"));

        assert!(err.is_invalid_argument());
        assert_eq!(err.to_string(), "Workspace ID cannot be nil");
    }

    #[test]
    fn test_not_found() {
        let err = Error::not_found(eyre!("Could not find workspace with ID: {}", Uuid::nil()));

        assert!(err.is_not_found());
        assert_eq!(
            err.to_string(),
            "Could not find workspace with ID: 00000000-0000-0000-0000-000000000000"
        );
    }
}
