use crate::solutions::{backup_credentials_fixture, InMemoryStorage};
use hermione_nexus::{
    definitions::BackupProviderKind, operations::DeleteBackupCredentialsOperation, Error, Result,
};

struct DeleteBackupCredentialsOperationTestContext {
    storage: InMemoryStorage,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(DeleteBackupCredentialsOperationTestContext) -> Result<()>,
{
    let storage = InMemoryStorage::default();

    test_fn(DeleteBackupCredentialsOperationTestContext { storage })
}

#[test]
fn it_deletes_backup_credentials() -> Result<()> {
    with_context(|ctx| {
        let DeleteBackupCredentialsOperationTestContext { storage } = ctx;

        let credentials = backup_credentials_fixture(Default::default());
        storage.insert_backup_credentials(credentials)?;

        assert_eq!(storage.count_backup_credentials()?, 1);

        DeleteBackupCredentialsOperation {
            find_provider: &storage,
            delete_provider: &storage,
        }
        .execute(&BackupProviderKind::Notion)?;

        assert_eq!(storage.count_backup_credentials()?, 0);

        Ok(())
    })
}

#[test]
fn it_returns_not_found_error() -> Result<()> {
    with_context(|ctx| {
        let DeleteBackupCredentialsOperationTestContext { storage } = ctx;

        let result = DeleteBackupCredentialsOperation {
            find_provider: &storage,
            delete_provider: &storage,
        }
        .execute(&BackupProviderKind::Notion);

        assert!(matches!(result, Err(Error::NotFound(_))));

        Ok(())
    })
}
