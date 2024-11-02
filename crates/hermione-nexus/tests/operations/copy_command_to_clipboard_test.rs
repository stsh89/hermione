use crate::{
    clipboard::MockClipboardProvider,
    fixtures::{command_fixture, workspace_fixture, CommandFixtureParameters},
    storage::InMemoryStorageProvider,
};
use hermione_nexus::{operations::CopyCommandToClipboardOperation, Result};

#[test]
fn it_copies_command_to_clipboard() -> Result<()> {
    let storage = InMemoryStorageProvider::new();
    let clipboard = MockClipboardProvider::new();

    let workspace = workspace_fixture(Default::default())?;
    let command = command_fixture(
        &workspace,
        CommandFixtureParameters {
            program: Some("ping 1.1.1.1".to_string()),
            ..Default::default()
        },
    )?;

    storage.insert_workspace(&workspace)?;
    storage.insert_command(&command)?;

    CopyCommandToClipboardOperation {
        find_command_provider: &storage,
        clipboard_provider: &clipboard,
    }
    .execute(command.id())?;

    assert_eq!(clipboard.text()?.as_deref(), Some("ping 1.1.1.1"));

    Ok(())
}
