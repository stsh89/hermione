use crate::{
    prelude::*,
    support::{self, InMemoryStorage},
};
use hermione_nexus::{
    definitions::Workspace,
    operations::{UpdateWorkspaceOperation, UpdateWorkspaceParameters},
};

#[derive(Default)]
struct UpdateWorkspaceTestCase {
    storage: InMemoryStorage,
    operation: Operation<Workspace>,
}

impl TestCase for UpdateWorkspaceTestCase {
    fn inspect_operation_results(&self, parameters: Table) {
        self.operation.assert_is_success();

        let workspace_table = parameters.get_table("storage").get_table("workspace");
        let id = workspace_table.get_uuid("id");

        let workspace = support::get_workspace(&self.storage, id);
        support::assert_workspace(&workspace, workspace_table);

        let workspace = self.operation.result().as_ref().unwrap();
        support::assert_workspace(workspace, workspace_table);
    }

    fn execute_operation(&mut self, parameters: Table) {
        let workspace_id = parameters.get_uuid("id");
        let name = parameters.get_text("name");
        let location = parameters.get_text("location");

        let result = UpdateWorkspaceOperation {
            find_workspace_provider: &self.storage,
            update_workspace_provider: &self.storage,
        }
        .execute(UpdateWorkspaceParameters {
            id: &workspace_id.into(),
            location: Some(location.to_string()),
            name: name.to_string(),
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
fn it_updates_workspace() {
    let mut test_case = UpdateWorkspaceTestCase::default();

    test_case.setup(table! {
        [storage.workspace]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        location = "/home/ironman"
        name = "Ironman"
    });

    test_case.execute_operation(table! {
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        location = "C:\\"
        name = "Avenger"
    });

    test_case.inspect_operation_results(table! {
        [storage.workspace]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        location = "C:\\"
        name = "Avenger"
    });
}
