use crate::{prelude::*, support::{InMemoryStorage}};
use hermione_nexus::{definitions::Workspace, operations::DeleteWorkspaceOperation, Error, Result};
use uuid::Uuid;

struct DeleteWorkspaceOperationTestCase {
    storage: InMemoryStorage,
    workspace: Workspace,
}

impl TestCase for DeleteWorkspaceOperationTestCase {
    fn assert_operation_success(&self, parameters: Table) {
        todo!()
    }

    fn execute_operation(&mut self, parameters: Table) {
        todo!()
    }
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(DeleteWorkspaceOperationTestCase) -> Result<()>,
{
    let storage = InMemoryStorage::default();

    let workspace = workspace_fixture(Default::default())?;
    let command = command_fixture(&workspace, Default::default())?;

    storage.insert_workspace(&workspace)?;
    storage.insert_command(&command)?;

    test_fn(DeleteWorkspaceOperationTestCase { storage, workspace })
}

#[test]
fn it_deletes_workspace() -> Result<()> {
    with_context(|ctx| {
        let DeleteWorkspaceOperationTestCase { storage, workspace } = ctx;

        assert_eq!(storage.count_workspaces()?, 1);

        DeleteWorkspaceOperation {
            find_workspace_provider: &storage,
            delete_workspace_commands_provider: &storage,
            delete_workspace_provider: &storage,
        }
        .execute(workspace.id())?;

        assert_eq!(storage.count_workspaces()?, 0);

        Ok(())
    })
}

#[test]
fn it_deletes_workspace_commands() -> Result<()> {
    with_context(|ctx| {
        let DeleteWorkspaceOperationTestCase { storage, workspace } = ctx;

        assert_eq!(storage.count_commands()?, 1);

        DeleteWorkspaceOperation {
            find_workspace_provider: &storage,
            delete_workspace_commands_provider: &storage,
            delete_workspace_provider: &storage,
        }
        .execute(workspace.id())?;

        assert_eq!(storage.count_commands()?, 0);

        Ok(())
    })
}

#[test]
fn it_returns_not_found_error() -> Result<()> {
    with_context(|ctx| {
        let DeleteWorkspaceOperationTestCase {
            storage,
            workspace: _,
        } = ctx;

        assert_eq!(storage.count_workspaces()?, 1);

        let result = DeleteWorkspaceOperation {
            find_workspace_provider: &storage,
            delete_workspace_commands_provider: &storage,
            delete_workspace_provider: &storage,
        }
        .execute(&Uuid::nil().into());

        assert_eq!(storage.count_workspaces()?, 1);
        assert!(matches!(result, Err(Error::NotFound(_))));

        Ok(())
    })
}
