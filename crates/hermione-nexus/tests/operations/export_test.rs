use crate::solutions::{
    backup_credentials_fixture, command_fixture, workspace_fixture, InMemoryStorageProvider,
    MockBackupProvider, MockBackupProviderBuilder, MockBackupProviderParameters,
};
use hermione_nexus::{
    definitions::{BackupCredentials, BackupProviderKind},
    operations::ExportOperation,
    Error, Result,
};
use std::marker::PhantomData;

struct ExportOperationTestContext {
    backup: MockBackupProvider,
    backup_provider_builder: MockBackupProviderBuilder,
    storage: InMemoryStorageProvider,
    backup_credentials: BackupCredentials,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(ExportOperationTestContext) -> Result<()>,
{
    let backup_credentials = backup_credentials_fixture(Default::default());
    let storage = InMemoryStorageProvider::new();
    let backup_provider_builder = MockBackupProviderBuilder::default();
    let backup = MockBackupProvider::new(MockBackupProviderParameters {
        credentials: backup_credentials.clone(),
        workspaces: backup_provider_builder.workspaces(),
        commands: backup_provider_builder.commands(),
    });

    storage.insert_backup_credentials(backup_credentials.clone())?;

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
        backup_credentials,
    })
}

#[test]
fn it_exports() -> Result<()> {
    with_context(|ctx| {
        let ExportOperationTestContext {
            backup,
            ref backup_provider_builder,
            storage,
            backup_credentials,
        } = ctx;

        assert_eq!(backup.count_workspaces()?, 0);
        assert_eq!(backup.count_commands()?, 0);

        ExportOperation {
            backup_credentials_provider: &storage,
            backup_provider_builder,
            list_commands_provider: &storage,
            list_workspaces_provider: &storage,
            phantom_backup_provider_builder: PhantomData,
        }
        .execute(&backup_credentials.provider_kind())?;

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
            backup_credentials: _,
        } = ctx;

        assert_eq!(storage.count_backup_credentials()?, 0);

        let result = ExportOperation {
            backup_credentials_provider: &storage,
            backup_provider_builder,
            list_commands_provider: &storage,
            list_workspaces_provider: &storage,
            phantom_backup_provider_builder: PhantomData,
        }
        .execute(&BackupProviderKind::Unknown);

        assert!(matches!(result, Err(Error::NotFound(_))));

        Ok(())
    })
}
