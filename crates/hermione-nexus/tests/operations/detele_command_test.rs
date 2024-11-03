use crate::solutions::{command_fixture, workspace_fixture, InMemoryStorageProvider};
use hermione_nexus::{definitions::Command, operations::DeleteCommandOperation, Error, Result};
use uuid::Uuid;

struct DeleteCommandOperationTestContext {
    storage: InMemoryStorageProvider,
    command: Command,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(DeleteCommandOperationTestContext) -> Result<()>,
{
    let storage = InMemoryStorageProvider::new();

    let workspace = workspace_fixture(Default::default())?;
    let command = command_fixture(&workspace, Default::default())?;

    storage.insert_workspace(&workspace)?;
    storage.insert_command(&command)?;

    test_fn(DeleteCommandOperationTestContext { storage, command })
}

#[test]
fn it_deletes_command() -> Result<()> {
    with_context(|ctx| {
        let DeleteCommandOperationTestContext { storage, command } = ctx;

        assert_eq!(storage.commands()?.len(), 1);

        DeleteCommandOperation {
            find_command_provider: &storage,
            delete_command_provider: &storage,
        }
        .execute(command.id())?;

        assert_eq!(storage.commands()?.len(), 0);

        Ok(())
    })
}

#[test]
fn it_returns_not_found_error() -> Result<()> {
    with_context(|ctx| {
        let DeleteCommandOperationTestContext {
            storage,
            command: _,
        } = ctx;

        assert_eq!(storage.commands()?.len(), 1);

        let result = DeleteCommandOperation {
            find_command_provider: &storage,
            delete_command_provider: &storage,
        }
        .execute(&Uuid::nil().into());

        assert_eq!(storage.commands()?.len(), 1);
        assert!(matches!(result, Err(Error::NotFound(_))));

        Ok(())
    })
}
