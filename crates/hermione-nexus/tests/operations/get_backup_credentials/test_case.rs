use crate::support::{
    self, ExpectedNotionBackupCredentials, InMemoryStorage, NotionBackupCredentialsFixture,
};
use hermione_nexus::{
    definitions::{BackupCredentials, BackupProviderKind},
    operations::GetBackupCredentialsOperation,
    Error,
};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub fn assert_operation_success(
    operation_result: Result<BackupCredentials, Error>,
    expected: ExpectedNotionBackupCredentials,
) {
    match operation_result {
        Ok(credentials) => {
            support::assert_notion_backup_credentials(credentials, expected);
        }
        Err(error) => panic!(
            "Get backup credentials operation failed with error: {}",
            error
        ),
    }
}

pub fn execute_operation(
    backgournd: &Background,
    backup_provider_kind: BackupProviderKind,
) -> Result<BackupCredentials, Error> {
    let Background { storage } = backgournd;

    GetBackupCredentialsOperation { provider: storage }.execute(backup_provider_kind)
}

pub fn setup(backgournd: &Background, credentials: NotionBackupCredentialsFixture) {
    let Background { storage } = backgournd;

    support::insert_notion_backup_credentials(storage, credentials);
}
