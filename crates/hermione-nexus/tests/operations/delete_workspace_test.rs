use crate::solutions::{command_fixture, workspace_fixture, InMemoryStorageProvider};
use hermione_nexus::{definitions::Workspace, operations::DeleteWorkspaceOperation, Error, Result};
use uuid::Uuid;

struct DeleteWorkspaceOperationTestContext {
    storage: InMemoryStorageProvider,
    workspace: Workspace,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(DeleteWorkspaceOperationTestContext) -> Result<()>,
{
    let storage = InMemoryStorageProvider::new();

    let workspace = workspace_fixture(Default::default())?;
    let command = command_fixture(&workspace, Default::default())?;

    storage.insert_workspace(&workspace)?;
    storage.insert_command(&command)?;

    test_fn(DeleteWorkspaceOperationTestContext { storage, workspace })
}

#[test]
fn it_deletes_workspace() -> Result<()> {
    with_context(|ctx| {
        let DeleteWorkspaceOperationTestContext { storage, workspace } = ctx;

        assert_eq!(storage.workspaces_count()?, 1);

        DeleteWorkspaceOperation {
            find_workspace_provider: &storage,
            delete_workspace_commands_provider: &storage,
            delete_workspace_provider: &storage,
        }
        .execute(workspace.id())?;

        assert_eq!(storage.workspaces_count()?, 0);

        Ok(())
    })
}

#[test]
fn it_deletes_workspace_commands() -> Result<()> {
    with_context(|ctx| {
        let DeleteWorkspaceOperationTestContext { storage, workspace } = ctx;

        assert_eq!(storage.commands_count()?, 1);

        DeleteWorkspaceOperation {
            find_workspace_provider: &storage,
            delete_workspace_commands_provider: &storage,
            delete_workspace_provider: &storage,
        }
        .execute(workspace.id())?;

        assert_eq!(storage.commands_count()?, 0);

        Ok(())
    })
}

#[test]
fn it_returns_not_found_error() -> Result<()> {
    with_context(|ctx| {
        let DeleteWorkspaceOperationTestContext {
            storage,
            workspace: _,
        } = ctx;

        assert_eq!(storage.workspaces_count()?, 1);

        let result = DeleteWorkspaceOperation {
            find_workspace_provider: &storage,
            delete_workspace_commands_provider: &storage,
            delete_workspace_provider: &storage,
        }
        .execute(&Uuid::nil().into());

        assert_eq!(storage.workspaces_count()?, 1);
        assert!(matches!(result, Err(Error::NotFound(_))));

        Ok(())
    })
}
