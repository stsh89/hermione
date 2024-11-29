use crate::{
    prelude::*,
    support::{self, InMemoryStorage},
};
use hermione_nexus::{
    definitions::Command,
    operations::{UpdateCommandOperation, UpdateCommandParameters},
};

#[derive(Default)]
struct UpdateCommandTestCase {
    storage: InMemoryStorage,
    operation: Operation<Command>,
}

impl TestCase for UpdateCommandTestCase {
    fn inspect_operation_results(&self, parameters: Table) {
        self.operation.assert_is_success();

        let command_table = parameters.get_table("storage").get_table("command");
        let id = command_table.get_command_id("id");

        let command = support::get_command(&self.storage, id);
        support::assert_command(&command, command_table);

        let command = self.operation.result().as_ref().unwrap();
        support::assert_command(command, command_table);
    }

    fn execute_operation(&mut self, parameters: Table) {
        let id = parameters.get_command_id("id");
        let name = parameters.get_text("name");
        let program = parameters.get_text("program");

        let result = UpdateCommandOperation {
            find_command_provider: &self.storage,
            update_command_provider: &self.storage,
        }
        .execute(UpdateCommandParameters {
            id,
            program: program.to_string(),
            name: name.to_string(),
        });

        self.operation.set_result(result);
    }

    fn setup(&mut self, parameters: Table) {
        let storage_table = parameters.get_table("storage");

        support::insert_workspace(&self.storage, storage_table.get_table("workspace"));
        support::insert_command(&self.storage, storage_table.get_table("command"));
    }
}

#[test]
fn it_updates_command() {
    let mut test_case = UpdateCommandTestCase::default();

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
        id = "51280bfc-2eea-444a-8df9-a1e7158c2c6b"
        name = "List directory items"
        program = "GetChild-Item ."
    });

    test_case.inspect_operation_results(table! {
        [storage.command]
        id = "51280bfc-2eea-444a-8df9-a1e7158c2c6b"
        name = "List directory items"
        program = "GetChild-Item ."
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    });
}
