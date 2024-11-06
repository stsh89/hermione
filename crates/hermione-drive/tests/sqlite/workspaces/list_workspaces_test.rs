use chrono::Utc;
use hermione_drive::sqlite::{self, ListWorkspacesQuery};
use rusqlite::{Connection, Result};

use crate::solutions::{workspace_record_fixture, WorkspaceRecordFixtureParameters};

struct ListWorkspacesTestContest {
    conn: Connection,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(ListWorkspacesTestContest) -> Result<()>,
{
    let conn = Connection::open_in_memory()?;
    sqlite::create_workspaces_table(&conn)?;

    for workspace_number in 1..=8 {
        let record = workspace_record_fixture(WorkspaceRecordFixtureParameters {
            name: Some(format!("Workspace {}", workspace_number)),
            ..Default::default()
        });

        sqlite::insert_workspace(&conn, record)?;
    }

    test_fn(ListWorkspacesTestContest { conn })
}

#[test]
fn it_paginates_workspaces() -> Result<()> {
    with_context(|ctx| {
        let ListWorkspacesTestContest { conn } = ctx;

        let workspaces = sqlite::list_workspaces(
            &conn,
            ListWorkspacesQuery {
                name_contains: "",
                limit: 2,
                offset: 3,
            },
        )?;

        assert_eq!(
            workspaces.into_iter().map(|w| w.name).collect::<Vec<_>>(),
            vec!["Workspace 7", "Workspace 8"]
        );

        Ok(())
    })
}

#[test]
fn it_sorts_workspaces_by_last_access_time_and_name() -> Result<()> {
    with_context(|ctx| {
        let ListWorkspacesTestContest { conn } = ctx;

        let record = workspace_record_fixture(WorkspaceRecordFixtureParameters {
            name: Some("Workspace 9".to_string()),
            last_access_time: Utc::now().timestamp_nanos_opt(),
            ..Default::default()
        });

        sqlite::insert_workspace(&conn, record)?;

        let workspaces = sqlite::list_workspaces(
            &conn,
            ListWorkspacesQuery {
                name_contains: "",
                limit: 4,
                offset: 0,
            },
        )?;

        assert_eq!(
            workspaces.into_iter().map(|w| w.name).collect::<Vec<_>>(),
            vec!["Workspace 9", "Workspace 1", "Workspace 2", "Workspace 3",]
        );

        Ok(())
    })
}

#[test]
fn it_filters_workspaces_by_name() -> Result<()> {
    with_context(|ctx| {
        let ListWorkspacesTestContest { conn } = ctx;

        let workspaces = sqlite::list_workspaces(
            &conn,
            ListWorkspacesQuery {
                name_contains: "4",
                limit: 4,
                offset: 0,
            },
        )?;

        assert_eq!(
            workspaces.into_iter().map(|w| w.name).collect::<Vec<_>>(),
            vec!["Workspace 4",]
        );

        Ok(())
    })
}
