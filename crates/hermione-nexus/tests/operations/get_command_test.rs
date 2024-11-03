use crate::solutions::{command_fixture, workspace_fixture, InMemoryStorageProvider};
use hermione_nexus::{definitions::Command, operations::GetCommandOperation, Error, Result};
use uuid::Uuid;

pub struct GetCommandOperationTestContext {
    storage: InMemoryStorageProvider,
    command: Command,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(GetCommandOperationTestContext) -> Result<()>,
{
    let storage = InMemoryStorageProvider::new();
    let workspace = workspace_fixture(Default::default())?;
    let command = command_fixture(&workspace, Default::default())?;

    storage.insert_workspace(&workspace)?;
    storage.insert_command(&command)?;

    test_fn(GetCommandOperationTestContext { storage, command })
}

#[test]
fn it_returns_command() -> Result<()> {
    with_context(|ctx| {
        let GetCommandOperationTestContext { storage, command } = ctx;

        let found = GetCommandOperation { provider: &storage }.execute(command.id())?;

        assert_eq!(found.name(), command.name());

        Ok(())
    })
}

#[test]
fn it_returns_not_found_error() -> Result<()> {
    with_context(|ctx| {
        let GetCommandOperationTestContext {
            storage,
            command: _,
        } = ctx;

        let result = GetCommandOperation { provider: &storage }.execute(&Uuid::nil().into());

        assert!(matches!(result, Err(Error::NotFound(_))));

        Ok(())
    })
}
