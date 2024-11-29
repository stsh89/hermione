use crate::{
    prelude::*,
    support::{self, InMemoryStorage},
};
use hermione_nexus::{
    definitions::Command,
    operations::{CreateCommandOperation, CreateCommandParameters},
};

#[derive(Default)]
struct CreateCommandTestCase {
    storage: InMemoryStorage,
    operation: Operation<Command>,
}

impl TestCase for CreateCommandTestCase {
    fn inspect_operation_results(&self, parameters: Table) {
        self.operation.assert_is_success();

        let command = self.operation.result().as_ref().unwrap();
        let command = support::get_command(&self.storage, command.id());

        support::assert_command(
            &command,
            parameters.get_table("storage").get_table("command"),
        );
    }

    fn execute_operation(&mut self, parameters: Table) {
        let name = parameters.get_text("name");
        let program = parameters.get_text("program");
        let workspace_id = parameters.get_uuid("workspace_id");

        let result = CreateCommandOperation {
            storage_provider: &self.storage,
        }
        .execute(CreateCommandParameters {
            name: name.to_string(),
            program: program.to_string(),
            workspace_id: workspace_id.into(),
        });

        self.operation.set_result(result);
    }

    fn setup(&mut self, parameters: Table) {
        support::insert_workspace(
            &self.storage,
            parameters.get_table("storage").get_table("workspace"),
        );
    }
}

#[test]
fn it_creates_command() {
    let mut test_case = CreateCommandTestCase::default();

    test_case.setup(table! {
        [storage.workspace]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        name = "Ironman"
    });

    test_case.execute_operation(table! {
        name = "Ping"
        program = "ping 1.1.1.1"
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    });

    test_case.inspect_operation_results(table! {
        [storage.command]
        name = "Ping"
        program = "ping 1.1.1.1"
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    });
}
