use crate::{
    fixtures::{command_fixture, workspace_fixture, CommandFixtureParameters},
    storage::InMemoryStorageProvider,
};
use hermione_nexus::{
    operations::{UpdateCommandOperation, UpdateCommandParameters},
    Error, Result,
};
use uuid::Uuid;

#[test]
fn it_updates_command() -> Result<()> {
    let storage_provider = InMemoryStorageProvider::new();
    let workspace = workspace_fixture(Default::default())?;
    let command = command_fixture(
        &workspace,
        CommandFixtureParameters {
            name: Some("Test command".to_string()),
            program: Some("ping 1.1.1.1".to_string()),
            ..Default::default()
        },
    )?;

    storage_provider.insert_workspace(&workspace)?;
    storage_provider.insert_command(&command)?;

    let command = UpdateCommandOperation {
        find_provider: &storage_provider,
        update_provider: &storage_provider,
    }
    .execute(UpdateCommandParameters {
        id: command.id(),
        name: "Get child items".to_string(),
        program: "Get-ChildItem".to_string(),
    })?;

    assert_eq!(command.name(), "Get child items");
    assert_eq!(command.program(), "Get-ChildItem");
    assert_eq!(command.last_execute_time(), None);

    Ok(())
}

#[test]
fn it_returns_command_not_found_error() -> Result<()> {
    let storage_provider = InMemoryStorageProvider::new();

    let result = UpdateCommandOperation {
        find_provider: &storage_provider,
        update_provider: &storage_provider,
    }
    .execute(UpdateCommandParameters {
        id: &Uuid::new_v4().into(),
        name: "Get child items".to_string(),
        program: "Get-ChildItem".to_string(),
    });

    match result {
        Err(error) => assert!(matches!(error, Error::NotFound(_))),
        Ok(_) => unreachable!(),
    };

    Ok(())
}
