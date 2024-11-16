use crate::solutions::{
    backup_credentials_fixture, command_fixture, workspace_fixture, InMemoryStorage, MockNotionBackup,
    MockNotionBackupBuilder, MockNotionBackupParameters, MockNotionStorage,
};
use hermione_nexus::{
    definitions::{BackupCredentials, BackupProviderKind},
    operations::ImportOperation,
    Error, Result,
};
use std::marker::PhantomData;

struct ImportOperationTestContext {
    storage: InMemoryStorage,
    backup: MockNotionBackup,
    backup_builder: MockNotionBackupBuilder,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(ImportOperationTestContext) -> Result<()>,
{
    let storage = InMemoryStorage::default();
    let storage_backup = MockNotionStorage::default();

    let credentials = backup_credentials_fixture(Default::default());
    storage.insert_backup_credentials(credentials.clone())?;

    let BackupCredentials::Notion(credentials) = credentials;
    let backup_builder =
        MockNotionBackupBuilder::new(storage_backup.commands(), storage_backup.workspaces());
    let backup = MockNotionBackup::new(MockNotionBackupParameters {
        credentials,
        workspaces: storage_backup.workspaces(),
        commands: storage_backup.commands(),
    });

    for _ in 1..=2 {
        let workspace = workspace_fixture(Default::default())?;
        backup.insert_workspace(&workspace)?;

        for _ in 1..=3 {
            let command = command_fixture(&workspace, Default::default())?;
            backup.insert_command(&command)?;
        }
    }

    test_fn(ImportOperationTestContext {
        storage,
        backup,
        backup_builder,
    })
}

#[test]
fn it_inserts_missing_workspaces_and_commands() -> Result<()> {
    with_context(|ctx| {
        let ImportOperationTestContext {
            storage,
            backup,
            backup_builder,
        } = ctx;

        assert_eq!(backup.count_commands()?, 6);
        assert_eq!(backup.count_workspaces()?, 2);
        assert_eq!(storage.count_workspaces()?, 0);
        assert_eq!(storage.count_commands()?, 0);

        ImportOperation {
            backup_credentials_provider: &storage,
            upsert_commands_provider: &storage,
            upsert_workspaces_provider: &storage,
            backup_provider_builder: &backup_builder,
            backup_provider: PhantomData,
        }
        .execute(&BackupProviderKind::Notion)?;

        assert_eq!(storage.count_commands()?, 6);
        assert_eq!(storage.count_workspaces()?, 2);

        Ok(())
    })
}

#[test]
fn it_updates_existing_workspaces_and_commands() -> Result<()> {
    with_context(|ctx| {
        let ImportOperationTestContext {
            storage,
            backup,
            backup_builder,
        } = ctx;

        let mut workspace = backup.workspaces()?.first().cloned().unwrap();
        let mut command = backup
            .commands()?
            .into_iter()
            .find(|command| command.workspace_id() == workspace.id())
            .unwrap();

        workspace.set_name("Renamed workspaces".to_string());
        command.set_program("Renamed program".to_string());

        storage.insert_workspace(&workspace)?;
        storage.insert_command(&command)?;

        assert_eq!(backup.count_commands()?, 6);
        assert_eq!(backup.count_workspaces()?, 2);

        assert_eq!(storage.count_commands()?, 1);
        assert_eq!(storage.count_workspaces()?, 1);

        let found_workspace = storage.get_workspace(workspace.id())?.unwrap();
        let found_command = storage.get_command(command.id())?.unwrap();

        assert!(found_workspace.name() == "Renamed workspaces");
        assert!(found_command.program() == "Renamed program");

        ImportOperation {
            backup_credentials_provider: &storage,
            upsert_commands_provider: &storage,
            upsert_workspaces_provider: &storage,
            backup_provider_builder: &backup_builder,
            backup_provider: PhantomData,
        }
        .execute(&BackupProviderKind::Notion)?;

        assert_eq!(storage.count_commands()?, 6);
        assert_eq!(storage.count_workspaces()?, 2);

        let found_workspace = storage.get_workspace(workspace.id())?.unwrap();
        let found_command = storage.get_command(command.id())?.unwrap();

        assert!(found_workspace.name() != "Renamed workspaces");
        assert!(found_command.program() != "Renamed program");

        Ok(())
    })
}

#[test]
fn it_returns_not_found_error() -> Result<()> {
    with_context(|ctx| {
        let ImportOperationTestContext {
            storage,
            backup: _,
            backup_builder: ref backup_provider_builder,
        } = ctx;

        let res = ImportOperation {
            backup_credentials_provider: &storage,
            upsert_commands_provider: &storage,
            upsert_workspaces_provider: &storage,
            backup_provider_builder,
            backup_provider: PhantomData,
        }
        .execute(&BackupProviderKind::Unknown);

        assert!(matches!(res, Err(Error::NotFound(_))));

        Ok(())
    })
}
