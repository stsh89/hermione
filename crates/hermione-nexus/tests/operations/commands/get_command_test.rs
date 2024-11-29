use crate::{
    prelude::*,
    support::{self, InMemoryStorage},
};
use hermione_nexus::{definitions::Command, operations::GetCommandOperation};

#[derive(Default)]
struct GetCommandTestCase {
    storage: InMemoryStorage,
    operation: Operation<Command>,
}

impl TestCase for GetCommandTestCase {
    fn inspect_operation_results(&self, parameters: Table) {
        self.operation.assert_is_success();

        let command_table = parameters.get_table("response").get_table("command");
        let command = self.operation.result().as_ref().unwrap();

        support::assert_command(command, command_table);
    }

    fn execute_operation(&mut self, parameters: Table) {
        let command_id = parameters.get_command_id("command_id");

        let result = GetCommandOperation {
            provider: &self.storage,
        }
        .execute(command_id);

        self.operation.set_result(result);
    }

    fn setup(&mut self, parameters: Table) {
        let storage_table = parameters.get_table("storage");

        support::insert_workspace(&self.storage, storage_table.get_table("workspace"));
        support::insert_command(&self.storage, storage_table.get_table("command"));
    }
}

#[test]
fn it_returns_command() {
    let mut test_case = GetCommandTestCase::default();

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
        [response.command]
        id = "51280bfc-2eea-444a-8df9-a1e7158c2c6b"
        name = "Ping"
        program = "ping 1.1.1.1"
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    });
}
