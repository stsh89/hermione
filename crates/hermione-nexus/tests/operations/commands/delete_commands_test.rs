use crate::{
    prelude::*,
    support::{self, InMemoryStorage},
};
use hermione_nexus::operations::{
    CommandsDeleteAttribute, DeleteCommandsOperation, DeleteCommandsParameters,
};

#[derive(Default)]
struct DeleteCommandsTestCase {
    storage: InMemoryStorage,
    operation: Operation<()>,
}

impl TestCase for DeleteCommandsTestCase {
    fn execute_operation(&mut self, parameters: Table) {
        let delete_attribute_table = parameters.get_table("delete_attribute");

        let delete_attribute = match delete_attribute_table.get_text("attribute_kind") {
            "workspace_id" => CommandsDeleteAttribute::WorkspaceId(
                delete_attribute_table.get_workspace_id("workspace_id"),
            ),
            kind => panic!("Unexpected delete attribute kind: {kind}"),
        };

        let result = DeleteCommandsOperation {
            delete_workspace_commands: &self.storage,
        }
        .execute(DeleteCommandsParameters { delete_attribute });

        self.operation.set_result(result);
    }

    fn inspect_operation_results(&self, expectations: Table) {
        self.operation.assert_is_success();

        let storage_table = expectations.get_table("storage");
        let commands_count = storage_table.get_table("commands").get_integer("count") as usize;

        support::assert_commands_count(&self.storage, commands_count);
    }

    fn setup(&mut self, parameters: Table) {
        let storage_table = parameters.get_table("storage");

        support::insert_workspace(&self.storage, storage_table.get_table("workspace"));
        support::insert_command(&self.storage, storage_table.get_table("command"));
    }
}

#[test]
fn it_deletes_commands_by_workspace_id() {
    let mut test_case = DeleteCommandsTestCase::default();

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
        [delete_attribute]
        attribute_kind = "workspace_id"
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    });

    test_case.inspect_operation_results(table! {
        [storage.commands]
        count = 0
    });
}
