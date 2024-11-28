use crate::{
    prelude::*,
    support::{self, InMemoryStorage, MockSystem},
};
use hermione_nexus::operations::{ExecuteProgramOperation, ExecuteProgramParameters};

#[derive(Default)]
struct ExecuteProgramTestCase {
    storage: InMemoryStorage,
    system: MockSystem,
    operation: Operation<()>,
}

impl TestCase for ExecuteProgramTestCase {
    fn inspect_operation_results(&self, parameters: Table) {
        self.operation.assert_is_success();

        let system_table = parameters.get_table("system");
        let location = system_table.get_text("location");
        let program = system_table.get_text("last_executed_program");

        support::assert_system_location(&self.system, Some(location));
        support::assert_system_program(&self.system, Some(program));
    }

    fn execute_operation(&mut self, parameters: Table) {
        let workspace_id = parameters.get_uuid("workspace_id");
        let program = parameters.get_text("program");

        let result = ExecuteProgramOperation {
            find_workspace: &self.storage,
            system: &self.system,
        }
        .execute(ExecuteProgramParameters {
            program,
            workspace_id: workspace_id.into(),
        });

        self.operation.set_result(result);
    }

    fn setup(&mut self, parameters: Table) {
        let storage_table = parameters.get_table("storage");

        support::insert_workspace(&self.storage, storage_table.get_table("workspace"));
    }
}

#[test]
fn it_returns_workspace() {
    let mut test_case = ExecuteProgramTestCase::default();

    test_case.setup(table! {
        [storage.workspace]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        location = "/home/ironman"
        name = "Ironman"
    });

    test_case.execute_operation(table! {
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        program = "ping 1.1.1.1"
    });

    test_case.inspect_operation_results(table! {
        [system]
        location = "/home/ironman"
        last_executed_program = "ping 1.1.1.1"
    });
}
