use crate::solutions::{
    command_fixture, workspace_fixture, CommandFixtureParameters, InMemoryStorageProvider,
    MockClipboardProvider,
};
use hermione_nexus::{
    definitions::Command, operations::CopyCommandToClipboardOperation, Error, Result,
};
use uuid::Uuid;

struct CopyCommandToClipboardOperationTestContext {
    clipboard: MockClipboardProvider,
    command: Command,
    storage: InMemoryStorageProvider,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(CopyCommandToClipboardOperationTestContext) -> Result<()>,
{
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

    test_fn(CopyCommandToClipboardOperationTestContext {
        storage,
        clipboard,
        command,
    })
}

#[test]
fn it_copies_command_to_clipboard() -> Result<()> {
    with_context(|ctx| {
        let CopyCommandToClipboardOperationTestContext {
            storage,
            clipboard,
            command,
        } = ctx;

        assert!(clipboard.content()?.is_none());

        CopyCommandToClipboardOperation {
            storage_provider: &storage,
            clipboard_provider: &clipboard,
        }
        .execute(command.id())?;

        assert_eq!(clipboard.content()?.as_deref(), Some("ping 1.1.1.1"));

        Ok(())
    })
}

#[test]
fn it_returns_not_found_error() -> Result<()> {
    with_context(|ctx| {
        let CopyCommandToClipboardOperationTestContext {
            storage,
            clipboard,
            command: _,
        } = ctx;

        clipboard.set_content("Get-ChildItem")?;

        let result = CopyCommandToClipboardOperation {
            storage_provider: &storage,
            clipboard_provider: &clipboard,
        }
        .execute(&Uuid::nil().into());

        assert!(matches!(result, Err(Error::NotFound(_))));
        assert_eq!(clipboard.content()?.as_deref(), Some("Get-ChildItem"));

        Ok(())
    })
}
