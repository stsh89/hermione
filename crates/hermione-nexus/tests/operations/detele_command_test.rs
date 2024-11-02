use crate::{
    fixtures::{command_fixture, workspace_fixture},
    storage::InMemoryStorageProvider,
};
use hermione_nexus::{operations::DeleteCommandOperation, Error, Result};
use uuid::Uuid;

#[test]
fn it_deletes_command() -> Result<()> {
    let storage = InMemoryStorageProvider::new();

    let workspace = workspace_fixture(Default::default())?;
    let command = command_fixture(&workspace, Default::default())?;

    storage.insert_workspace(&workspace)?;
    storage.insert_command(&command)?;

    assert_eq!(storage.commands()?.len(), 1);

    DeleteCommandOperation {
        find_command_provider: &storage,
        delete_command_provider: &storage,
    }
    .execute(command.id())?;

    assert_eq!(storage.commands()?.len(), 0);

    Ok(())
}

#[test]
fn it_returns_not_found_error() -> Result<()> {
    let storage = InMemoryStorageProvider::new();
    let id = Uuid::new_v4();

    let result = DeleteCommandOperation {
        find_command_provider: &storage,
        delete_command_provider: &storage,
    }
    .execute(&id.into());

    match result {
        Err(Error::NotFound(description)) => assert_eq!(description, format!("Command {{{id}}}")),
        _ => unreachable!(),
    };

    Ok(())
}
