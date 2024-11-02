use crate::{
    fixtures::{command_fixture, workspace_fixture, CommandFixtureParameters},
    storage::InMemoryStorageProvider,
};
use chrono::{TimeZone, Utc};
use hermione_nexus::{
    operations::{ListCommandsOperation, ListCommandsParameters},
    Error, Result,
};

#[test]
fn it_filters_commands_by_program() -> Result<()> {
    let storage = InMemoryStorageProvider::new();

    let workspace = workspace_fixture(Default::default())?;
    let command1 = command_fixture(
        &workspace,
        CommandFixtureParameters {
            program: Some("ping 1.1.1.1".to_string()),
            ..Default::default()
        },
    )?;
    let command2 = command_fixture(
        &workspace,
        CommandFixtureParameters {
            program: Some("Get-ChildItem".to_string()),
            ..Default::default()
        },
    )?;

    storage.insert_workspace(&workspace)?;
    storage.insert_command(&command1)?;
    storage.insert_command(&command2)?;

    let commands =
        ListCommandsOperation { provider: &storage }.execute(ListCommandsParameters {
            page_size: 10,
            page_number: 1,
            program_contains: Some("ing"),
            workspace_id: None,
        })?;

    assert_eq!(
        commands.iter().map(|c| c.program()).collect::<Vec<_>>(),
        vec![command1.program()]
    );

    Ok(())
}

#[test]
fn it_filters_command_by_workspace() -> Result<()> {
    let storage = InMemoryStorageProvider::new();

    let workspace1 = workspace_fixture(Default::default())?;
    let workspace2 = workspace_fixture(Default::default())?;

    let command1 = command_fixture(
        &workspace1,
        CommandFixtureParameters {
            program: Some("ping 1.1.1.1".to_string()),
            ..Default::default()
        },
    )?;

    let command2 = command_fixture(
        &workspace2,
        CommandFixtureParameters {
            program: Some("Get-ChildItem".to_string()),
            ..Default::default()
        },
    )?;

    storage.insert_workspace(&workspace1)?;
    storage.insert_workspace(&workspace2)?;
    storage.insert_command(&command1)?;
    storage.insert_command(&command2)?;

    let commands =
        ListCommandsOperation { provider: &storage }.execute(ListCommandsParameters {
            page_size: 10,
            page_number: 1,
            program_contains: None,
            workspace_id: Some(workspace2.id()),
        })?;

    assert_eq!(
        commands.iter().map(|c| c.program()).collect::<Vec<_>>(),
        vec![command2.program()]
    );

    Ok(())
}

#[test]
fn it_paginates() -> Result<()> {
    let storage = InMemoryStorageProvider::new();

    let workspace = workspace_fixture(Default::default())?;

    let command1 = command_fixture(
        &workspace,
        CommandFixtureParameters {
            program: Some("ls".to_string()),
            ..Default::default()
        },
    )?;

    let command2 = command_fixture(
        &workspace,
        CommandFixtureParameters {
            program: Some("ping 1.1.1.1".to_string()),
            last_execute_time: Some(Utc.with_ymd_and_hms(2024, 10, 30, 10, 10, 10).unwrap()),
            ..Default::default()
        },
    )?;

    let command3 = command_fixture(
        &workspace,
        CommandFixtureParameters {
            program: Some("cat".to_string()),
            ..Default::default()
        },
    )?;

    storage.insert_workspace(&workspace)?;
    storage.insert_command(&command1)?;
    storage.insert_command(&command2)?;
    storage.insert_command(&command3)?;

    let commands =
        ListCommandsOperation { provider: &storage }.execute(ListCommandsParameters {
            page_size: 2,
            page_number: 2,
            program_contains: None,
            workspace_id: None,
        })?;

    assert_eq!(
        commands.iter().map(|c| c.program()).collect::<Vec<_>>(),
        vec![command1.program()]
    );

    Ok(())
}

#[test]
fn it_validates_page_size() -> Result<()> {
    let storage = InMemoryStorageProvider::new();

    let result = ListCommandsOperation { provider: &storage }.execute(ListCommandsParameters {
        program_contains: None,
        workspace_id: None,
        page_number: 1,
        page_size: 0,
    });

    match result {
        Err(Error::InvalidArgument(description)) => {
            assert_eq!(description, "Page size must be greater than 0")
        }
        _ => unreachable!(),
    };

    Ok(())
}

#[test]
fn it_validates_page_number() -> Result<()> {
    let storage = InMemoryStorageProvider::new();

    let result = ListCommandsOperation { provider: &storage }.execute(ListCommandsParameters {
        program_contains: None,
        workspace_id: None,
        page_number: 0,
        page_size: 10,
    });

    match result {
        Err(Error::InvalidArgument(description)) => {
            assert_eq!(description, "Page number must be greater than 0")
        }
        _ => unreachable!(),
    };

    Ok(())
}
