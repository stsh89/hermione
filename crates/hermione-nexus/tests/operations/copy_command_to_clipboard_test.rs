use crate::{
    prelude::*,
    support::{self, InMemoryStorage, MockSystem},
};
use hermione_nexus::operations::CopyCommandToClipboardOperation;

#[derive(Default)]
struct CopyCommandToClipboardTestCase {
    system: MockSystem,
    operation: Operation<()>,
    storage: InMemoryStorage,
}

impl TestCase for CopyCommandToClipboardTestCase {
    fn setup(&mut self, parameters: Table) {
        support::insert_workspace(
            &self.storage,
            parameters.get_table("storage").get_table("workspace"),
        );

        support::insert_command(
            &self.storage,
            parameters.get_table("storage").get_table("command"),
        );
    }

    fn execute_operation(&mut self, parameters: Table) {
        let command_id = parameters.get_uuid("command_id").into();

        let result = CopyCommandToClipboardOperation {
            storage_provider: &self.storage,
            clipboard_provider: &self.system,
        }
        .execute(&command_id);

        self.operation.set_result(result);
    }

    fn assert_operation(&self, parameters: Table) {
        self.operation.assert_is_success();

        let clipboard_table = parameters.get_table("clipboard");
        let expected_clipboard_content = clipboard_table.get_text("content");
        let got_clipboard_content = support::get_clipboard_content(&self.system);

        assert_eq!(got_clipboard_content, expected_clipboard_content);
    }
}

#[test]
fn it_copies_command_to_clipboard() {
    let mut test_case = CopyCommandToClipboardTestCase::default();

    test_case.setup(table! {
        [storage.workspace]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        location = "/home/ironman"
        name = "Ironman"

        [storage.command]
        id = "51280bfc-2eea-444a-8df9-a1e7158c2c6b"
        name = "Ping"
        program = "ping 1.1.1.1"
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    });

    test_case.execute_operation(table! {
        command_id = "51280bfc-2eea-444a-8df9-a1e7158c2c6b"
    });

    test_case.assert_operation(table! {
        [clipboard]
        content = "ping 1.1.1.1"
    });
}
