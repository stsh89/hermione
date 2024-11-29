use crate::{
    prelude::*,
    support::{self, InMemoryStorage},
};
use hermione_nexus::operations::DeleteWorkspaceOperation;

#[derive(Default)]
struct DeleteWorkspaceTestCase {
    storage: InMemoryStorage,
    operation: Operation<()>,
}

impl TestCase for DeleteWorkspaceTestCase {
    fn inspect_operation_results(&self, parameters: Table) {
        self.operation.assert_is_success();

        let storage_table = parameters.get_table("storage");
        let commands_count = storage_table.get_table("commands").get_integer("count") as usize;
        let workspaces_count = storage_table.get_table("workspaces").get_integer("count") as usize;

        support::assert_commands_count(&self.storage, commands_count);
        support::assert_workspaces_count(&self.storage, workspaces_count);
    }

    fn execute_operation(&mut self, parameters: Table) {
        let workspace_id = parameters.get_uuid("workspace_id").into();

        let result = DeleteWorkspaceOperation {
            find_workspace_provider: &self.storage,
            delete_workspace_provider: &self.storage,
        }
        .execute(&workspace_id);

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
fn it_deletes_workspace() {
    let mut test_case = DeleteWorkspaceTestCase::default();

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
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    });

    test_case.inspect_operation_results(table! {
        [storage.workspaces]
        count = 0

        [storage.commands]
        count = 1
    });
}
