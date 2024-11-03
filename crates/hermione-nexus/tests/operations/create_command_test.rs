use crate::{fixtures::workspace_fixture, storage::InMemoryStorageProvider};
use hermione_nexus::{
    definitions::Workspace,
    operations::{CreateCommandOperation, CreateCommandParameters},
    Result,
};

struct CreateCommandOperationTestContext {
    storage: InMemoryStorageProvider,
    workspace: Workspace,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(CreateCommandOperationTestContext) -> Result<()>,
{
    let storage = InMemoryStorageProvider::new();
    let workspace = workspace_fixture(Default::default())?;

    storage.insert_workspace(&workspace)?;

    test_fn(CreateCommandOperationTestContext { storage, workspace })
}

#[test]
fn it_creates_command() -> Result<()> {
    with_context(|ctx| {
        let CreateCommandOperationTestContext { storage, workspace } = ctx;

        assert_eq!(storage.commands_count()?, 0);

        let command =
            CreateCommandOperation { provider: &storage }.execute(CreateCommandParameters {
                name: "Test command".to_string(),
                program: "ping 1.1.1.1".to_string(),
                workspace_id: workspace.id().clone(),
            })?;

        assert_eq!(storage.commands_count()?, 1);
        assert_eq!(command.name(), "Test command");
        assert_eq!(command.program(), "ping 1.1.1.1");
        assert_eq!(command.last_execute_time(), None);
        assert_eq!(command.workspace_id(), workspace.id());

        Ok(())
    })
}
