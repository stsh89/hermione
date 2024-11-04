pub enum BackupProviderKind {
    Notion,
}

#[derive(Clone)]
pub enum BackupCredentials {
    Notion(NotionBackupCredentials),
}

#[derive(Clone)]
pub struct NotionBackupCredentials {
    api_key: String,
    workspaces_database_id: String,
    commands_database_id: String,
}

pub struct NotionBackupCredentialsParameters {
    pub api_key: String,
    pub workspaces_database_id: String,
    pub commands_database_id: String,
}

impl BackupCredentials {
    pub fn provider_kind(&self) -> BackupProviderKind {
        match self {
            BackupCredentials::Notion(_) => BackupProviderKind::Notion,
        }
    }

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

    pub fn workspaces_database_id(&self) -> &str {
        &self.workspaces_database_id
    }

    pub fn commands_database_id(&self) -> &str {
        &self.commands_database_id
    }
}

impl BackupProviderKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Notion => "Notion",
        }
    }
}