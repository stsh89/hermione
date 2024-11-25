use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
pub enum BackupProviderKind {
    Notion,
    Unknown,
}

#[derive(Clone)]
pub enum BackupCredentials {
    Notion(NotionBackupCredentials),
}

#[derive(Clone)]
pub struct NotionBackupCredentials {
    api_key: String,
    commands_database_id: String,
    workspaces_database_id: String,
}

pub struct NotionBackupCredentialsParameters {
    pub api_key: String,
    pub commands_database_id: String,
    pub workspaces_database_id: String,
}

impl BackupCredentials {
    pub fn notion(parameters: NotionBackupCredentialsParameters) -> Self {
        Self::Notion(NotionBackupCredentials::new(parameters))
    }
}

impl NotionBackupCredentials {
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn new(parameters: NotionBackupCredentialsParameters) -> Self {
        Self {
            api_key: parameters.api_key,
            workspaces_database_id: parameters.workspaces_database_id,
            commands_database_id: parameters.commands_database_id,
        }
    }

    pub fn commands_database_id(&self) -> &str {
        &self.commands_database_id
    }

    pub fn workspaces_database_id(&self) -> &str {
        &self.workspaces_database_id
    }
}

impl Display for BackupProviderKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupProviderKind::Notion => write!(f, "Notion"),
            BackupProviderKind::Unknown => write!(f, "Unknown"),
        }
    }
}
