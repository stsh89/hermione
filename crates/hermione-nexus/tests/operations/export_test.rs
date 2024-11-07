use crate::solutions::{
    backup_credentials_fixture, command_fixture, workspace_fixture, InMemoryStorage, MockBackup,
    MockBackupBuilder, MockBackupParameters, MockStorageBackup,
};
use hermione_nexus::{
    definitions::{BackupCredentials, BackupProviderKind},
    operations::ExportOperation,
    Error, Result,
};
use std::marker::PhantomData;

struct ExportOperationTestContext {
    backup: MockBackup,
    backup_provider_builder: MockBackupBuilder,
    storage: InMemoryStorage,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(ExportOperationTestContext) -> Result<()>,
{
    let storage = InMemoryStorage::default();
    let storage_backup = MockStorageBackup::default();

    let backup_credentials = backup_credentials_fixture(Default::default());
    storage.insert_backup_credentials(backup_credentials.clone())?;

    let BackupCredentials::Notion(credentials) = backup_credentials.clone();
    let backup_provider_builder =
        MockBackupBuilder::new(storage_backup.commands(), storage_backup.workspaces());
    let backup = MockBackup::new(MockBackupParameters {
        credentials,
        workspaces: storage_backup.workspaces(),
        commands: storage_backup.commands(),
    });

    for _ in 1..=2 {
        let workspace = workspace_fixture(Default::default())?;
        storage.insert_workspace(&workspace)?;

        for _ in 1..=3 {
            let command = command_fixture(&workspace, Default::default())?;

            storage.insert_command(&command)?;
        }
    }

    test_fn(ExportOperationTestContext {
        backup,
        backup_provider_builder,
        storage,
    })
}

#[test]
fn it_exports() -> Result<()> {
    with_context(|ctx| {
        let ExportOperationTestContext {
            backup,
            ref backup_provider_builder,
            storage,
        } = ctx;

        assert_eq!(backup.count_workspaces()?, 0);
        assert_eq!(backup.count_commands()?, 0);

        ExportOperation {
            backup_credentials_provider: &storage,
            backup_provider_builder,
            list_commands_provider: &storage,
            list_workspaces_provider: &storage,
            backup_provider: PhantomData,
        }
        .execute(&BackupProviderKind::Notion)?;

        assert_eq!(backup.count_workspaces()?, 2);
        assert_eq!(backup.count_commands()?, 6);

        Ok(())
    })
}

#[test]
fn it_returns_not_found_error() -> Result<()> {
    with_context(|ctx| {
        let ExportOperationTestContext {
            storage,
            backup: _,
            ref backup_provider_builder,
        } = ctx;

        let result = ExportOperation {
            backup_credentials_provider: &storage,
            backup_provider_builder,
            list_commands_provider: &storage,
            list_workspaces_provider: &storage,
            backup_provider: PhantomData,
        }
        .execute(&BackupProviderKind::Unknown);

        assert!(matches!(result, Err(Error::NotFound(_))));

        Ok(())
    })
}
