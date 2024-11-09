use crate::support::{
    self, command_record_fixture, workspace_record_fixture, CommandRecordFixtureParameters,
};
use hermione_drive::sqlite::{self, CommandRecord, OptionalValue};
use rusqlite::{Connection, Result};

struct UpdateCommandTestContext {
    conn: Connection,
    command: CommandRecord,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(UpdateCommandTestContext) -> Result<()>,
{
    let conn = Connection::open_in_memory()?;

    sqlite::create_workspaces_table_if_not_exists(&conn)?;
    sqlite::create_commands_table_if_not_exists(&conn)?;

    let workspace = workspace_record_fixture(Default::default());
    sqlite::insert_workspace(&conn, workspace.clone())?;

    let command = command_record_fixture(
        &workspace,
        CommandRecordFixtureParameters {
            last_execute_time: Some(10),
            name: Some("Test command".to_string()),
            program: Some("echo \"Hello, world!\"".to_string()),
            ..Default::default()
        },
    );

    sqlite::insert_command(&conn, command.clone())?;

    test_fn(UpdateCommandTestContext { conn, command })
}

#[test]
fn it_updates_command_name() -> Result<()> {
    with_context(|ctx| {
        let UpdateCommandTestContext { conn, command } = ctx;

        assert_eq!(command.name, "Test command");

        let count = sqlite::update_command(
            &conn,
            sqlite::UpdateCommandQueryOptions {
                id: command.id,
                last_execute_time: None,
                name: Some("Spaceship".to_string()),
                program: None,
            },
        )?;

        assert_eq!(count, 1);

        let command = support::query_command(&conn, &command.id)?;

        assert_eq!(command.name, "Spaceship");
        assert_eq!(command.last_execute_time, Some(10));
        assert_eq!(command.program, "echo \"Hello, world!\"");

        Ok(())
    })
}

#[test]
fn it_updates_command_program() -> Result<()> {
    with_context(|ctx| {
        let UpdateCommandTestContext { conn, command } = ctx;

        assert_eq!(command.program, "echo \"Hello, world!\"");

        let count = sqlite::update_command(
            &conn,
            sqlite::UpdateCommandQueryOptions {
                id: command.id,
                last_execute_time: None,
                name: None,
                program: Some("echo \"Hello, universe!\"".to_string()),
            },
        )?;

        assert_eq!(count, 1);

        let command = support::query_command(&conn, &command.id)?;

        assert_eq!(command.name, "Test command");
        assert_eq!(command.last_execute_time, Some(10));
        assert_eq!(command.program, "echo \"Hello, universe!\"");

        Ok(())
    })
}

#[test]
fn it_updates_last_execute_time() -> Result<()> {
    with_context(|ctx| {
        let UpdateCommandTestContext { conn, command } = ctx;

        assert_eq!(command.last_execute_time, Some(10));

        let count = sqlite::update_command(
            &conn,
            sqlite::UpdateCommandQueryOptions {
                id: command.id,
                last_execute_time: Some(sqlite::OptionalValue::Value(20)),
                name: None,
                program: None,
            },
        )?;

        assert_eq!(count, 1);

        let command = support::query_command(&conn, &command.id)?;

        assert_eq!(command.name, "Test command");
        assert_eq!(command.last_execute_time, Some(20));
        assert_eq!(command.program, "echo \"Hello, world!\"");

        Ok(())
    })
}

#[test]
fn it_updates_last_execute_time_with_none() -> Result<()> {
    with_context(|ctx| {
        let UpdateCommandTestContext { conn, command } = ctx;

        assert_eq!(command.last_execute_time, Some(10));

        let count = sqlite::update_command(
            &conn,
            sqlite::UpdateCommandQueryOptions {
                id: command.id,
                last_execute_time: Some(OptionalValue::Null),
                name: None,
                program: None,
            },
        )?;

        assert_eq!(count, 1);

        let command = support::query_command(&conn, &command.id)?;

        assert_eq!(command.name, "Test command");
        assert_eq!(command.last_execute_time, None);
        assert_eq!(command.program, "echo \"Hello, world!\"");

        Ok(())
    })
}

#[test]
fn it_does_not_update_command_when_nothing_changed() -> Result<()> {
    with_context(|ctx| {
        let UpdateCommandTestContext { conn, command } = ctx;

        assert_eq!(command.last_execute_time, Some(10));
        assert_eq!(command.name, "Test command");
        assert_eq!(command.program, "echo \"Hello, world!\"");

        let count = sqlite::update_command(
            &conn,
            sqlite::UpdateCommandQueryOptions {
                id: command.id,
                last_execute_time: None,
                name: None,
                program: None,
            },
        )?;

        assert_eq!(count, 0);

        let command = support::query_command(&conn, &command.id)?;

        assert_eq!(command.name, "Test command");
        assert_eq!(command.last_execute_time, Some(10));
        assert_eq!(command.program, "echo \"Hello, world!\"");

        Ok(())
    })
}
