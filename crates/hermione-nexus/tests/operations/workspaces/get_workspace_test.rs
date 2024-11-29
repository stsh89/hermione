use crate::{
    prelude::*,
    support::{self, InMemoryStorage},
};
use hermione_nexus::{definitions::Workspace, operations::GetWorkspaceOperation};

#[derive(Default)]
struct GetWorkspaceTestCase {
    storage: InMemoryStorage,
    operation: Operation<Workspace>,
}

impl TestCase for GetWorkspaceTestCase {
    fn inspect_operation_results(&self, parameters: Table) {
        self.operation.assert_is_success();

        let response_table = parameters.get_table("response");
        let workspace_table = response_table.get_table("workspace");
        let workspace = self.operation.result().as_ref().unwrap();

        support::assert_workspace(workspace, &workspace_table);
    }

    fn execute_operation(&mut self, parameters: Table) {
        let workspace_id = parameters.get_workspace_id("workspace_id");

        let result = GetWorkspaceOperation {
            provider: &self.storage,
        }
        .execute(workspace_id);

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
fn it_returns_workspace() {
    let mut test_case = GetWorkspaceTestCase::default();

    test_case.setup(table! {
        [storage.workspace]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        location = "/home/ironman"
        name = "Ironman"
    });

    test_case.execute_operation(table! {
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    });

    test_case.inspect_operation_results(table! {
        [response.workspace]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        location = "/home/ironman"
        name = "Ironman"
    });
}
