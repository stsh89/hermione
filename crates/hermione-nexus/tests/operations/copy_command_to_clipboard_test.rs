use crate::support::{self, InMemoryStorage, MockClipboard};
use hermione_nexus::{operations::CopyCommandToClipboardOperation, Error, Result};
use serde_json::json;
use serde_json::Value as Json;
use uuid::Uuid;

struct CopyCommandToClipboardTestContext {
    storage: InMemoryStorage,
    clipboard: MockClipboard,
    error: Option<Error>,
}

impl CopyCommandToClipboardTestContext {
    fn assert_clipbard_contains_program(&self, program: &str) {
        assert_eq!(self.clipboard.content().unwrap().as_deref(), Some(program));
    }

    fn assert_clipbard_is_empty(&self) {
        assert_eq!(self.clipboard.content().unwrap(), None);
    }

    fn assert_error_contains_message(&self, message: &str) {
        assert_eq!(self.error.as_ref().unwrap().to_string(), message);
    }

    fn copy_command_to_clipboard(&self, command_id: &str) -> Result<()> {
        let id: Uuid = command_id.parse().unwrap();

        CopyCommandToClipboardOperation {
            storage_provider: &self.storage,
            clipboard_provider: &self.clipboard,
        }
        .execute(&id.into())?;

        Ok(())
    }

    fn try_to_copy_command_to_clipboard(&mut self, command_id: &str) -> Result<()> {
        let id: Uuid = command_id.parse().unwrap();

        let error = CopyCommandToClipboardOperation {
            storage_provider: &self.storage,
            clipboard_provider: &self.clipboard,
        }
        .execute(&id.into())
        .unwrap_err();

        self.error = Some(error);

        Ok(())
    }

    fn with_background() -> Self {
        let context = Self {
            storage: InMemoryStorage::empty(),
            clipboard: MockClipboard::empty(),
            error: None,
        };

        storage_contains_workspace(
            &context,
            json!({
                "id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                "name": "Ironman",
                "location": "/home/ironman",
            }),
        );

        storage_contains_command(
            &context,
            json!({
                "id": "51280bfc-2eea-444a-8df9-a1e7158c2c6b",
                "workspace_id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                "name": "Ping",
                "program": "ping 1.1.1.1",
            }),
        );

        context
    }
}

fn storage_contains_workspace(context: &CopyCommandToClipboardTestContext, parameters: Json) {
    support::insert_workspace(&context.storage, parameters);
}

fn storage_contains_command(context: &CopyCommandToClipboardTestContext, parameters: Json) {
    support::insert_command(&context.storage, parameters);
}

#[test]
fn it_copies_command_to_clipboard() -> Result<()> {
    let context = CopyCommandToClipboardTestContext::with_background();

    context.copy_command_to_clipboard("51280bfc-2eea-444a-8df9-a1e7158c2c6b")?;
    context.assert_clipbard_contains_program("ping 1.1.1.1");

    Ok(())
}

#[test]
fn it_returns_not_found_error() -> Result<()> {
    let mut context = CopyCommandToClipboardTestContext::with_background();

    context.try_to_copy_command_to_clipboard("00000000-0000-0000-0000-000000000000")?;

    context.assert_clipbard_is_empty();
    context
        .assert_error_contains_message("Command {00000000-0000-0000-0000-000000000000} not found");

    Ok(())
}
