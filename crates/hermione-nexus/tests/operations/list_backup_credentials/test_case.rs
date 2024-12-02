use crate::support::{
    self, ExpectedBackupCredentials, InMemoryStorage, NotionBackupCredentialsFixture,
};
use hermione_nexus::{
    definitions::BackupCredentials, operations::ListBackupCredentialsOperation, Error,
};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub fn assert_operation_success(
    operation_result: Result<Vec<BackupCredentials>, Error>,
    expected: Vec<ExpectedBackupCredentials>,
) {
    match operation_result {
        Ok(credentials) => {
            support::assert_backup_credentials_list(credentials, expected);
        }
        Err(error) => panic!(
            "List backup credentials operation failed with error: {}",
            error
        ),
    }
}

pub fn execute_operation(backgournd: &Background) -> Result<Vec<BackupCredentials>, Error> {
    let Background { storage } = backgournd;

    ListBackupCredentialsOperation { provider: storage }.execute()
}

pub fn setup(backgournd: &Background, credentials: NotionBackupCredentialsFixture) {
    let Background { storage } = backgournd;

    support::insert_notion_backup_credentials(storage, credentials);
}
