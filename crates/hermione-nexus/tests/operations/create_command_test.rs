use crate::{fixtures::workspace_fixture, storage::InMemoryStorageProvider};
use hermione_nexus::{
    operations::{CreateCommandOperation, CreateCommandParameters},
    Result,
};

#[test]
fn it_creates_command() -> Result<()> {
    let provider = &InMemoryStorageProvider::new();
    let workspace = workspace_fixture(Default::default())?;

    provider.insert_workspace(&workspace)?;

    let command = CreateCommandOperation { provider }.execute(CreateCommandParameters {
        name: "Test command".to_string(),
        program: "ping 1.1.1.1".to_string(),
        workspace_id: workspace.id().clone(),
    })?;

    assert_eq!(command.name(), "Test command");
    assert_eq!(command.program(), "ping 1.1.1.1");
    assert_eq!(command.last_execute_time(), None);
    assert_eq!(**command.workspace_id(), **workspace.id());

    Ok(())
}
