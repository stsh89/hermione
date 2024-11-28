use crate::{
    prelude::*,
    support::{self, InMemoryStorage},
};
use hermione_nexus::operations::DeleteCommandOperation;

#[derive(Default)]
struct DeleteCommandTestCase {
    storage: InMemoryStorage,
    operation: Operation<()>,
}

impl TestCase for DeleteCommandTestCase {
    fn inspect_operation_results(&self, parameters: Table) {
        self.operation.assert_is_success();

        let storage_table = parameters.get_table("storage");
        let commands_count = storage_table.get_table("commands").get_integer("count") as usize;

        support::assert_commands_count(&self.storage, commands_count);
    }

    fn execute_operation(&mut self, parameters: Table) {
        let command_id = parameters.get_uuid("command_id").into();

        let result = DeleteCommandOperation {
            find_provider: &self.storage,
            delete_provider: &self.storage,
        }
        .execute(&command_id);

        self.operation.set_result(result);
    }

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
}

#[test]
fn it_deletes_command() {
    let mut test_case = DeleteCommandTestCase::default();

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
        [storage.commands]
        count = 0
    });
}
