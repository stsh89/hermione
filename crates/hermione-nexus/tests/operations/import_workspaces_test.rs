use crate::solutions::{
    backup_credentials_fixture, workspace_fixture, InMemoryStorage, MockBackup, MockBackupBuilder,
    MockBackupParameters, MockStorageBackup,
};
use hermione_nexus::{
    definitions::{BackupCredentials, BackupProviderKind},
    operations::{ImportWorkspacesOperation, ImportWorkspacesOperationParameters},
    Result,
};

struct ImportWorkspacesOperationTestContext {
    backup: MockBackup,
    backup_builder: MockBackupBuilder,
    storage: InMemoryStorage,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(ImportWorkspacesOperationTestContext) -> Result<()>,
{
    let storage = InMemoryStorage::default();
    let storage_backup = MockStorageBackup::default();

    let credentials = backup_credentials_fixture(Default::default());
    storage.insert_backup_credentials(credentials.clone())?;

    let BackupCredentials::Notion(credentials) = credentials;
    let backup_builder =
        MockBackupBuilder::new(storage_backup.commands(), storage_backup.workspaces());
    let backup = MockBackup::new(MockBackupParameters {
        credentials,
        workspaces: storage_backup.workspaces(),
        commands: storage_backup.commands(),
    });

    let workspace = workspace_fixture(Default::default())?;
    backup.insert_workspace(&workspace)?;

    test_fn(ImportWorkspacesOperationTestContext {
        backup,
        backup_builder,
        storage,
    })
}

#[test]
fn it_creates_workspaces() -> Result<()> {
    with_context(|ctx| {
        let ImportWorkspacesOperationTestContext {
            backup,
            backup_builder,
            storage,
        } = ctx;

        assert_eq!(backup.count_workspaces()?, 1);
        assert_eq!(storage.count_workspaces()?, 0);

        ImportWorkspacesOperation::new(ImportWorkspacesOperationParameters {
            backup_credentials_provider: &storage,
            upsert_workspaces_provider: &storage,
            backup_provider_builder: &backup_builder,
        })
        .execute(BackupProviderKind::Notion)?;

        assert_eq!(storage.count_workspaces()?, 1);

        Ok(())
    })
}
