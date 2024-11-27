use eyre::Report;

#[derive(Debug, thiserror::Error)]
#[error("{source:?}")]
pub struct Error {
    kind: ErrorKind,
    source: Report,
}

#[derive(Debug)]
enum ErrorKind {
    InvalidArgument,
    NotFound,
    Service(ServiceKind),
}

#[derive(Debug)]
enum ServiceKind {
    Backup,
    Storage,
    System,
}

impl Error {
    pub fn backup(source: Report) -> Self {
        Self {
            kind: ErrorKind::Service(ServiceKind::Backup),
            source,
        }
    }

    pub fn invalid_argument(source: Report) -> Self {
        Self {
            kind: ErrorKind::InvalidArgument,
            source,
        }
    }

    pub fn is_backup(&self) -> bool {
        matches!(self.kind, ErrorKind::Service(ServiceKind::Backup))
    }

    pub fn is_invalid_argument(&self) -> bool {
        matches!(self.kind, ErrorKind::InvalidArgument)
    }

    pub fn is_not_found(&self) -> bool {
        matches!(self.kind, ErrorKind::NotFound)
    }

    pub fn is_service(&self) -> bool {
        matches!(self.kind, ErrorKind::Service(_))
    }

    pub fn is_storage(&self) -> bool {
        matches!(self.kind, ErrorKind::Service(ServiceKind::Storage))
    }

    pub fn is_system(&self) -> bool {
        matches!(self.kind, ErrorKind::Service(ServiceKind::System))
    }

    pub fn not_found(source: Report) -> Self {
        Self {
            kind: ErrorKind::NotFound,
            source,
        }
    }

    pub fn storage(source: Report) -> Self {
        Self {
            kind: ErrorKind::Service(ServiceKind::Storage),
            source,
        }
    }

    pub fn system(source: Report) -> Self {
        Self {
            kind: ErrorKind::Service(ServiceKind::System),
            source,
        }
    }
}
