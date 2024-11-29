use crate::{
    prelude::*,
    support::{self, InMemoryStorage, MockSystem},
};
use hermione_nexus::operations::VisitWorkspaceLocationOperation;

#[derive(Default)]
struct VisitWorkspaceLocationTestCase {
    operation: Operation<()>,
    storage: InMemoryStorage,
    system: MockSystem,
}

impl TestCase for VisitWorkspaceLocationTestCase {
    fn setup(&mut self, parameters: Table) {
        support::insert_workspace(
            &self.storage,
            parameters.get_table("storage").get_table("workspace"),
        );
    }

    fn execute_operation(&mut self, parameters: Table) {
        let workspace_id = parameters.get_workspace_id("workspace_id");

        let result = VisitWorkspaceLocationOperation {
            find_workspace: &self.storage,
            system_provider: &self.system,
        }
        .execute(workspace_id);

        self.operation.set_result(result);
    }

    fn inspect_operation_results(&self, parameters: Table) {
        self.operation.assert_is_success();

        let system_table = parameters.get_table("system");
        let expected_system_location = system_table.get_text("location");
        let current_system_location = support::get_file_system_location(&self.system);

        assert_eq!(
            current_system_location.as_deref(),
            Some(expected_system_location)
        );
    }
}

#[test]
fn it_changes_working_directory() {
    let mut test_case = VisitWorkspaceLocationTestCase::default();

    test_case.setup(table! {
        [storage.workspace]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        name = "Ironman"
        location = "/home/ironman"
    });

    test_case.execute_operation(table! {
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    });

    test_case.inspect_operation_results(table! {
        [system]
        location = "/home/ironman"
    });
}
