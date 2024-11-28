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
        let storage_table = parameters.get_table("storage");

        support::insert_workspace(&self.storage, storage_table.get_table("workspace"));
        support::insert_command(&self.storage, storage_table.get_table("command"));
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

    fn inspect_operation_results(&self, parameters: Table) {
        self.operation.assert_is_success();

        let system_table = parameters.get_table("system");
        let expected_clipboard_content = system_table.get_text("clipboard");

        support::assert_clipboard_content(&self.system, expected_clipboard_content);
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

    test_case.inspect_operation_results(table! {
        [system]
        clipboard = "ping 1.1.1.1"
    });
}
