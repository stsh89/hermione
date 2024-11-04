use crate::solutions::{
    backup_credentials_fixture, command_fixture, workspace_fixture, InMemoryStorageProvider,
    MockBackupProvider, MockBackupProviderBuilder, MockBackupProviderParameters,
};
use hermione_nexus::{definitions::BackupProviderKind, operations::ImportOperation, Error, Result};
use std::marker::PhantomData;

struct ImportOperationTestContext {
    storage: InMemoryStorageProvider,
    backup: MockBackupProvider,
    backup_provider_builder: MockBackupProviderBuilder,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(ImportOperationTestContext) -> Result<()>,
{
    let credentials = backup_credentials_fixture(Default::default());
    let storage = InMemoryStorageProvider::new();
    let backup_provider_builder = MockBackupProviderBuilder::default();
    let backup = MockBackupProvider::new(MockBackupProviderParameters {
        credentials: credentials.clone(),
        workspaces: backup_provider_builder.workspaces(),
        commands: backup_provider_builder.commands(),
    });

    storage.insert_backup_credentials(credentials)?;

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
        backup_provider_builder,
    })
}

#[test]
fn it_inserts_missing_workspaces_and_commands() -> Result<()> {
    with_context(|ctx| {
        let ImportOperationTestContext {
            storage,
            backup,
            ref backup_provider_builder,
        } = ctx;

        assert_eq!(backup.count_commands()?, 6);
        assert_eq!(backup.count_workspaces()?, 2);
        assert_eq!(storage.count_workspaces()?, 0);
        assert_eq!(storage.count_commands()?, 0);

        ImportOperation {
            backup_credentials_provider: &storage,
            upsert_commands_provider: &storage,
            upsert_workspaces_provider: &storage,
            backup_provider_builder,
            phantom_backup_provider_builder: PhantomData,
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
            ref backup_provider_builder,
        } = ctx;

        let mut workspace = backup.workspaces()?.first().cloned().unwrap();
        let mut command = backup
            .commands()?
            .into_iter()
            .filter(|command| command.workspace_id() == workspace.id())
            .next()
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
            backup_provider_builder,
            phantom_backup_provider_builder: PhantomData,
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
            ref backup_provider_builder,
        } = ctx;

        assert_eq!(storage.count_backup_credentials()?, 0);

        let res = ImportOperation {
            backup_credentials_provider: &storage,
            upsert_commands_provider: &storage,
            upsert_workspaces_provider: &storage,
            backup_provider_builder,
            phantom_backup_provider_builder: PhantomData,
        }
        .execute(&BackupProviderKind::Unknown);

        assert!(matches!(res, Err(Error::NotFound(_))));

        Ok(())
    })
}
