use std::rc::Rc;

use crate::support::{
    self, ExpectedNotionBackupCredentials, InMemoryStorage, MockNotionBuilder, MockNotionStorage,
};
use hermione_nexus::{
    definitions::BackupCredentials,
    operations::{SaveBackupCredentialsOperation, SaveBackupCredentialsOperationParameters},
    Error,
};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub fn assert_operation_success(operation_result: Result<(), Error>) {
    match operation_result {
        Ok(()) => {}
        Err(error) => panic!(
            "Save backup credentials operation failed with error: {}",
            error
        ),
    }
}

pub fn assert_storage_contains_notion_backup_credentials(
    background: &Background,
    expected: ExpectedNotionBackupCredentials,
) {
    support::assert_stored_notion_backup_credentials(&background.storage, expected)
}

pub fn execute_operation(
    background: &Background,
    backup_credentials: BackupCredentials,
) -> Result<(), Error> {
    let Background { storage } = background;

    SaveBackupCredentialsOperation::new(SaveBackupCredentialsOperationParameters {
        save_provider: storage,
        backup_provider_builder: &MockNotionBuilder {
            storage: Rc::new(MockNotionStorage::empty()),
        },
    })
    .execute(&backup_credentials)
}
