use crate::solutions::{
    command_fixture, page_number, page_size, workspace_fixture, CommandFixtureParameters,
    InMemoryStorage,
};
use chrono::Utc;
use hermione_nexus::{
    definitions::Workspace,
    operations::{ListCommandsOperation, ListCommandsParameters},
    Result,
};

struct ListCommandsOperationTestContext {
    storage: InMemoryStorage,
    workspace: Workspace,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(ListCommandsOperationTestContext) -> Result<()>,
{
    let storage = InMemoryStorage::default();

    let workspace = workspace_fixture(Default::default())?;

    storage.insert_workspace(&workspace)?;

    for program_number in 1..=3 {
        let command = command_fixture(
            &workspace,
            CommandFixtureParameters {
                program: Some(format!("Get-ChildItem {program_number}")),
                ..Default::default()
            },
        )?;

        storage.insert_command(&command)?;
    }

    let workspace = workspace_fixture(Default::default())?;

    storage.insert_workspace(&workspace)?;

    for program_number in 1..=3 {
        let command = command_fixture(
            &workspace,
            CommandFixtureParameters {
                program: Some(format!("ping {program_number}")),
                ..Default::default()
            },
        )?;

        storage.insert_command(&command)?;
    }

    test_fn(ListCommandsOperationTestContext { storage, workspace })
}

#[test]
fn it_filters_commands_by_workspace() -> Result<()> {
    with_context(|ctx| {
        let ListCommandsOperationTestContext { storage, workspace } = ctx;

        let commands =
            ListCommandsOperation { provider: &storage }.execute(ListCommandsParameters {
                page_size: page_size(10)?,
                page_number: page_number(1)?,
                program_contains: None,
                workspace_id: Some(workspace.id()),
            })?;

        assert_eq!(
            commands.iter().map(|c| c.program()).collect::<Vec<_>>(),
            vec!["ping 1", "ping 2", "ping 3"]
        );

        Ok(())
    })
}

#[test]
fn it_filters_commands_by_program() -> Result<()> {
    with_context(|ctx| {
        let ListCommandsOperationTestContext {
            storage,
            workspace: _,
        } = ctx;

        let commands =
            ListCommandsOperation { provider: &storage }.execute(ListCommandsParameters {
                page_size: page_size(10)?,
                page_number: page_number(1)?,
                program_contains: Some("ping"),
                workspace_id: None,
            })?;

        assert_eq!(
            commands.iter().map(|c| c.program()).collect::<Vec<_>>(),
            vec!["ping 1", "ping 2", "ping 3"]
        );

        Ok(())
    })
}

#[test]
fn it_paginates() -> Result<()> {
    with_context(|ctx| {
        let ListCommandsOperationTestContext {
            storage,
            workspace: _,
        } = ctx;

        let commands =
            ListCommandsOperation { provider: &storage }.execute(ListCommandsParameters {
                page_number: page_number(2)?,
                page_size: page_size(2)?,
                program_contains: None,
                workspace_id: None,
            })?;

        assert_eq!(
            commands.iter().map(|c| c.program()).collect::<Vec<_>>(),
            vec!["Get-ChildItem 3", "ping 1"]
        );

        Ok(())
    })
}

#[test]
fn it_sorts_commands_by_last_execute_time_and_program() -> Result<()> {
    with_context(|ctx| {
        let ListCommandsOperationTestContext { storage, workspace } = ctx;

        let command = command_fixture(
            &workspace,
            CommandFixtureParameters {
                program: Some("trace".to_string()),
                last_execute_time: Some(Utc::now()),
                ..Default::default()
            },
        )?;

        storage.insert_command(&command)?;

        let commands =
            ListCommandsOperation { provider: &storage }.execute(ListCommandsParameters {
                page_size: page_size(3)?,
                page_number: page_number(1)?,
                program_contains: None,
                workspace_id: None,
            })?;

        assert_eq!(
            commands.iter().map(|w| w.program()).collect::<Vec<_>>(),
            vec!["trace", "Get-ChildItem 1", "Get-ChildItem 2",]
        );

        Ok(())
    })
}
