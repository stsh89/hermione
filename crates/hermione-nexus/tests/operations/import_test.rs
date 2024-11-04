use std::marker::PhantomData;

use crate::solutions::{
    backup_credentials_fixture, command_fixture, workspace_fixture, InMemoryStorageProvider,
    MockBackupProvider, MockBackupProviderBuilder,
};
use hermione_nexus::{
    definitions::{BackupProviderKind, Command, Workspace},
    operations::ImportOperation,
    Error, Result,
};

struct ImportOperationTestContext {
    storage: InMemoryStorageProvider,
    backup: MockBackupProvider,
    backup_provider_builder: MockBackupProviderBuilder,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(ImportOperationTestContext) -> Result<()>,
{
    let storage = InMemoryStorageProvider::new();

    let credentials = backup_credentials_fixture(Default::default());
    let backup = MockBackupProvider::new(credentials.clone());

    let mut workspaces: Vec<Workspace> = Vec::new();
    let mut commands: Vec<Command> = Vec::new();

    storage.insert_backup_credentials(credentials)?;

    for _ in 1..=2 {
        let workspace = workspace_fixture(Default::default())?;
        backup.insert_workspace(&workspace)?;

        for _ in 1..=3 {
            let command = command_fixture(&workspace, Default::default())?;

            backup.insert_command(&command)?;
            commands.push(command);
        }

        workspaces.push(workspace);
    }

    let mut backup_provider_builder = MockBackupProviderBuilder::new();

    backup_provider_builder.set_commands(commands);
    backup_provider_builder.set_workspaces(workspaces);

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

        assert_eq!(backup.commands_count()?, 6);
        assert_eq!(backup.workspaces_count()?, 2);
        assert_eq!(storage.workspaces_count()?, 0);
        assert_eq!(storage.commands_count()?, 0);

        ImportOperation {
            backup_credentials_provider: &storage,
            upsert_commands_provider: &storage,
            upsert_workspaces_provider: &storage,
            backup_provider_builder,
            phantom_backup_provider_builder: PhantomData,
        }
        .execute(&BackupProviderKind::Notion)?;

        assert_eq!(storage.commands_count()?, 6);
        assert_eq!(storage.workspaces_count()?, 2);

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

        assert_eq!(backup.commands_count()?, 6);
        assert_eq!(backup.workspaces_count()?, 2);

        assert_eq!(storage.commands_count()?, 1);
        assert_eq!(storage.workspaces_count()?, 1);

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

        assert_eq!(storage.commands_count()?, 6);
        assert_eq!(storage.workspaces_count()?, 2);

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

        storage.reset_backup_credentials()?;
        assert_eq!(storage.count_backup_credentials()?, 0);

        let res = ImportOperation {
            backup_credentials_provider: &storage,
            upsert_commands_provider: &storage,
            upsert_workspaces_provider: &storage,
            backup_provider_builder,
            phantom_backup_provider_builder: PhantomData,
        }
        .execute(&BackupProviderKind::Notion);

        assert!(matches!(res, Err(Error::NotFound(_))));

        Ok(())
    })
}
