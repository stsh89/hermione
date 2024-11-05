use crate::solutions::{workspace_record_fixture, WorkspaceRecordFixtureParameters};
use hermione_drive::sqlite::workspaces::{self, WorkspaceRecord};
use rusqlite::{Connection, Result};

struct RestoreWorkspacesTestContest {
    conn: Connection,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(RestoreWorkspacesTestContest) -> Result<()>,
{
    let conn = Connection::open_in_memory()?;
    workspaces::migrate(&conn)?;

    test_fn(RestoreWorkspacesTestContest { conn })
}

#[test]
fn it_inserts_workspace() -> Result<()> {
    with_context(|ctx| {
        let RestoreWorkspacesTestContest { conn } = ctx;

        assert_eq!(workspaces::count_workspaces(&conn)?, 0);

        let mut workspaces = vec![];

        for workspace_number in 1..=3 {
            let workspace = workspace_record_fixture(WorkspaceRecordFixtureParameters {
                name: Some(format!("Workspace {}", workspace_number)),
                ..Default::default()
            });

            workspaces.push(workspace);
        }

        workspaces::restore_workspaces(&conn, workspaces)?;

        let workspaces = workspaces::list_workspaces(
            &conn,
            workspaces::ListWorkspacesQuery {
                name_contains: "",
                limit: 10,
                offset: 0,
            },
        )?;

        assert_eq!(
            workspaces.into_iter().map(|w| w.name).collect::<Vec<_>>(),
            vec!["Workspace 1", "Workspace 2", "Workspace 3",]
        );

        Ok(())
    })
}

#[test]
fn it_udpates_existing_workspaces() -> Result<()> {
    with_context(|ctx| {
        let RestoreWorkspacesTestContest { conn } = ctx;

        let workspace1 = workspace_record_fixture(WorkspaceRecordFixtureParameters {
            name: Some("Workspace 1".to_string()),
            ..Default::default()
        });

        workspaces::insert_workspace(&conn, workspace1.clone())?;

        let workspace2 = workspace_record_fixture(WorkspaceRecordFixtureParameters {
            name: Some("Workspace 2".to_string()),
            ..Default::default()
        });

        let workspace1 = WorkspaceRecord {
            name: "Workspace 9".to_string(),
            ..workspace1
        };

        workspaces::restore_workspaces(&conn, vec![workspace1, workspace2])?;

        let workspaces = workspaces::list_workspaces(
            &conn,
            workspaces::ListWorkspacesQuery {
                name_contains: "",
                limit: 10,
                offset: 0,
            },
        )?;

        assert_eq!(
            workspaces.into_iter().map(|w| w.name).collect::<Vec<_>>(),
            vec!["Workspace 2", "Workspace 9",]
        );

        Ok(())
    })
}
