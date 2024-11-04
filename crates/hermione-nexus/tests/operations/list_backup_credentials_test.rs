use hermione_nexus::{operations::ListBackupCredentialsOperation, Result};

use crate::solutions::{backup_credentials_fixture, InMemoryStorageProvider};

struct ListBackupCredentialsOperationTestContext {
    storage: InMemoryStorageProvider,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(ListBackupCredentialsOperationTestContext) -> Result<()>,
{
    let storage = InMemoryStorageProvider::new();

    let backup_credentials = backup_credentials_fixture();
    storage.insert_backup_credentials(backup_credentials)?;

    test_fn(ListBackupCredentialsOperationTestContext { storage })
}

#[test]
fn it_lists_backup_credentials() -> Result<()> {
    with_context(|ctx| {
        let ListBackupCredentialsOperationTestContext { storage } = ctx;

        let backup_credentials = ListBackupCredentialsOperation { provider: &storage }.execute()?;

        assert_eq!(backup_credentials.len(), 1);

        Ok(())
    })
}
