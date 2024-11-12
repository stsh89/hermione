mod core {
    pub use hermione_nexus::{
        definitions::{BackupCredentials, BackupProviderKind, NotionBackupCredentialsParameters},
        Error, Result,
    };
}

use ratatui::widgets::ListItem;

impl<'a> From<&BackupProviderKind> for ListItem<'a> {
    fn from(value: &BackupProviderKind) -> Self {
        ListItem::new(value.as_str().to_string())
    }
}

impl From<core::BackupCredentials> for BackupProviderKind {
    fn from(value: core::BackupCredentials) -> Self {
        match value {
            core::BackupCredentials::Notion(_) => BackupProviderKind::Notion,
        }
    }
}

impl From<BackupProviderKind> for core::BackupProviderKind {
    fn from(value: BackupProviderKind) -> Self {
        match value {
            BackupProviderKind::Notion => core::BackupProviderKind::Notion,
        }
    }
}

impl From<core::BackupCredentials> for NotionBackupCredentials {
    fn from(value: core::BackupCredentials) -> Self {
        match value {
            core::BackupCredentials::Notion(credentials) => NotionBackupCredentials {
                api_key: credentials.api_key().to_string(),
                workspaces_database_id: credentials.workspaces_database_id().to_string(),
                commands_database_id: credentials.commands_database_id().to_string(),
            },
        }
    }
}

impl TryFrom<BackupCredentials> for core::BackupCredentials {
    type Error = core::Error;

    fn try_from(value: BackupCredentials) -> core::Result<Self> {
        let credentials = match value {
            BackupCredentials::Notion(presenter) => {
                let NotionBackupCredentials {
                    api_key,
                    workspaces_database_id,
                    commands_database_id,
                } = presenter;

                core::BackupCredentials::notion(core::NotionBackupCredentialsParameters {
                    api_key,
                    commands_database_id,
                    workspaces_database_id,
                })
            }
        };

        Ok(credentials)
    }
}
