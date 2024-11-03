use crate::solutions::{
    command_fixture, workspace_fixture, CommandFixtureParameters, InMemoryStorageProvider,
    MockSystemProvider,
};
use hermione_nexus::{definitions::Command, operations::ExecuteCommandOperation, Error, Result};
use uuid::Uuid;

struct ExecuteCommandOperationTestContext {
    storage: InMemoryStorageProvider,
    system: MockSystemProvider,
    command: Command,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(ExecuteCommandOperationTestContext) -> Result<()>,
{
    let storage = InMemoryStorageProvider::new();
    let system = MockSystemProvider::new();

    let workspace = workspace_fixture(Default::default())?;
    let command = command_fixture(
        &workspace,
        CommandFixtureParameters {
            program: Some("ping 1.1.1.1".to_string()),
            ..Default::default()
        },
    )?;

    storage.insert_workspace(&workspace)?;
    storage.insert_command(&command)?;

    test_fn(ExecuteCommandOperationTestContext {
        storage,
        system,
        command,
    })
}

#[test]
fn it_executes_command() -> Result<()> {
    with_context(|ctx| {
        let ExecuteCommandOperationTestContext {
            storage,
            system,
            command,
        } = ctx;

        assert!(system.last_executed_program()?.is_none());

        ExecuteCommandOperation {
            find_command_provider: &storage,
            track_command_provider: &storage,
            track_workspace_provider: &storage,
            system_provider: &system,
        }
        .execute(command.id())?;

        assert_eq!(
            system.last_executed_program()?.as_deref(),
            Some("ping 1.1.1.1")
        );

        Ok(())
    })
}

#[test]
fn it_tracks_command_execute_time() -> Result<()> {
    with_context(|ctx| {
        let ExecuteCommandOperationTestContext {
            storage,
            system,
            command,
        } = ctx;

        assert!(storage.get_command_execute_time(command.id())?.is_none());

        ExecuteCommandOperation {
            find_command_provider: &storage,
            track_command_provider: &storage,
            track_workspace_provider: &storage,
            system_provider: &system,
        }
        .execute(command.id())?;

        assert!(storage.get_command_execute_time(command.id())?.is_some());

        Ok(())
    })
}

#[test]
fn it_tracks_workspace_access_time() -> Result<()> {
    with_context(|ctx| {
        let ExecuteCommandOperationTestContext {
            storage,
            system,
            command,
        } = ctx;

        assert!(storage
            .get_workspace_access_time(command.workspace_id())?
            .is_none());

        ExecuteCommandOperation {
            find_command_provider: &storage,
            track_command_provider: &storage,
            track_workspace_provider: &storage,
            system_provider: &system,
        }
        .execute(command.id())?;

        assert!(storage
            .get_workspace_access_time(command.workspace_id())?
            .is_some());

        Ok(())
    })
}

#[test]
fn it_returns_not_found_error() -> Result<()> {
    with_context(|ctx| {
        let ExecuteCommandOperationTestContext {
            storage,
            system,
            command: _,
        } = ctx;

        system.set_program("Get-ChildItem")?;

        let result = ExecuteCommandOperation {
            find_command_provider: &storage,
            track_command_provider: &storage,
            track_workspace_provider: &storage,
            system_provider: &system,
        }
        .execute(&Uuid::nil().into());

        assert_eq!(
            system.last_executed_program()?.as_deref(),
            Some("Get-ChildItem")
        );
        assert!(matches!(result, Err(Error::NotFound(_))));

        Ok(())
    })
}
