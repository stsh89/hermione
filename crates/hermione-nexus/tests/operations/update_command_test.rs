use crate::solutions::{
    command_fixture, workspace_fixture, CommandFixtureParameters, InMemoryStorageProvider,
};
use hermione_nexus::{
    definitions::Command,
    operations::{UpdateCommandOperation, UpdateCommandParameters},
    Error, Result,
};
use uuid::Uuid;

struct UpdateCommandOperationTestContext {
    storage: InMemoryStorageProvider,
    command: Command,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(UpdateCommandOperationTestContext) -> Result<()>,
{
    let storage = InMemoryStorageProvider::new();
    let workspace = workspace_fixture(Default::default())?;
    let command = command_fixture(
        &workspace,
        CommandFixtureParameters {
            name: Some("Test command".to_string()),
            program: Some("ping 1.1.1.1".to_string()),
            ..Default::default()
        },
    )?;

    storage.insert_workspace(&workspace)?;
    storage.insert_command(&command)?;

    test_fn(UpdateCommandOperationTestContext { storage, command })
}

#[test]
fn it_updates_command() -> Result<()> {
    with_context(|ctx| {
        let UpdateCommandOperationTestContext { storage, command } = ctx;

        assert_eq!(command.name(), "Test command");
        assert_eq!(command.program(), "ping 1.1.1.1");

        let command = UpdateCommandOperation {
            find_command_provider: &storage,
            update_command_provider: &storage,
        }
        .execute(UpdateCommandParameters {
            id: command.id(),
            name: "Get child items".to_string(),
            program: "Get-ChildItem".to_string(),
        })?;

        assert_eq!(command.name(), "Get child items");
        assert_eq!(command.program(), "Get-ChildItem");

        Ok(())
    })
}

#[test]
fn it_returns_not_found_error() -> Result<()> {
    with_context(|ctx| {
        let UpdateCommandOperationTestContext {
            storage,
            command: _,
        } = ctx;

        let result = UpdateCommandOperation {
            find_command_provider: &storage,
            update_command_provider: &storage,
        }
        .execute(UpdateCommandParameters {
            id: &Uuid::nil().into(),
            name: "Get child items".to_string(),
            program: "Get-ChildItem".to_string(),
        });

        assert!(matches!(result, Err(Error::NotFound(_))));

        Ok(())
    })
}
