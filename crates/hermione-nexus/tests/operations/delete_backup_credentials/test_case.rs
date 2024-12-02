use crate::support::{self, InMemoryStorage, NotionBackupCredentialsFixture};
use hermione_nexus::{
    definitions::BackupProviderKind, operations::DeleteBackupCredentialsOperation, Error,
};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub fn assert_operation_success(operation_result: Result<(), Error>) {
    match operation_result {
        Ok(()) => {}
        Err(error) => panic!(
            "Delete backup credentials operation failed with error: {}",
            error
        ),
    }
}

pub fn assert_storage_does_not_contain_notion_backup_credentials(background: &Background) {
    let credentials = support::maybe_get_notion_backup_credentials(&background.storage);

    assert!(credentials.is_none());
}

pub fn execute_operation(
    backgournd: &Background,
    backup_provider_kind: BackupProviderKind,
) -> Result<(), Error> {
    let Background { storage } = backgournd;

    DeleteBackupCredentialsOperation {
        find_provider: storage,
        delete_provider: storage,
    }
    .execute(backup_provider_kind)
}

pub fn setup(backgournd: &Background, credentials: NotionBackupCredentialsFixture) {
    let Background { storage } = backgournd;

    support::insert_notion_backup_credentials(storage, credentials);
}
