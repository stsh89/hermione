use crate::{
    fixtures::{command_fixture, workspace_fixture},
    storage::InMemoryStorageProvider,
};
use hermione_nexus::{operations::DeleteWorkspaceOperation, Error, Result};
use uuid::Uuid;

#[test]
fn it_deletes_workspace() -> Result<()> {
    let storage = InMemoryStorageProvider::new();

    let workspace = workspace_fixture(Default::default())?;

    storage.insert_workspace(&workspace)?;

    assert_eq!(storage.workspaces()?.len(), 1);

    DeleteWorkspaceOperation {
        find_workspace_provider: &storage,
        delete_workspace_commands_provider: &storage,
        delete_workspace_provider: &storage,
    }
    .execute(workspace.id())?;

    assert!(storage.workspaces()?.is_empty());

    Ok(())
}

#[test]
fn it_deletes_workspace_commands() -> Result<()> {
    let storage = InMemoryStorageProvider::new();

    let workspace1 = workspace_fixture(Default::default())?;
    let command1 = command_fixture(&workspace1, Default::default())?;
    let command2 = command_fixture(&workspace1, Default::default())?;

    let workspace2 = workspace_fixture(Default::default())?;
    let command3 = command_fixture(&workspace2, Default::default())?;

    storage.insert_workspace(&workspace1)?;
    storage.insert_workspace(&workspace2)?;
    storage.insert_command(&command1)?;
    storage.insert_command(&command2)?;
    storage.insert_command(&command3)?;

    assert_eq!(storage.workspaces()?.len(), 2);
    assert_eq!(storage.commands()?.len(), 3);

    DeleteWorkspaceOperation {
        find_workspace_provider: &storage,
        delete_workspace_commands_provider: &storage,
        delete_workspace_provider: &storage,
    }
    .execute(workspace1.id())?;

    assert_eq!(
        storage
            .workspaces()?
            .iter()
            .map(|w| w.name())
            .collect::<Vec<_>>(),
        vec![workspace2.name()]
    );

    assert_eq!(
        storage
            .commands()?
            .iter()
            .map(|c| c.name())
            .collect::<Vec<_>>(),
        vec![command3.name()]
    );

    Ok(())
}

#[test]
fn it_returns_not_found_error() -> Result<()> {
    let storage = InMemoryStorageProvider::new();
    let id = Uuid::new_v4();

    let result = DeleteWorkspaceOperation {
        find_workspace_provider: &storage,
        delete_workspace_commands_provider: &storage,
        delete_workspace_provider: &storage,
    }
    .execute(&id.into());

    match result {
        Err(Error::NotFound(description)) => {
            assert_eq!(description, format!("Workspace {{{}}}", id))
        }
        _ => unreachable!(),
    };

    Ok(())
}
