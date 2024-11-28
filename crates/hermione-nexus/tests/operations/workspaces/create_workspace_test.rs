use crate::{
    prelude::*,
    support::{self, InMemoryStorage},
};
use hermione_nexus::{
    definitions::Workspace,
    operations::{CreateWorkspaceOperation, CreateWorkspaceParameters},
};

#[derive(Default)]
struct CreateWorkspaceTestCase {
    storage: InMemoryStorage,
    operation: Operation<Workspace>,
}

impl TestCase for CreateWorkspaceTestCase {
    fn inspect_operation_results(&self, parameters: Table) {
        self.operation.assert_is_success();

        let workspace = self.operation.result().as_ref().unwrap();
        let workspace = support::get_workspace(&self.storage, **workspace.id());

        support::assert_workspace(
            &workspace,
            parameters.get_table("storage").get_table("workspace"),
        );
    }

    fn execute_operation(&mut self, parameters: Table) {
        let name = parameters.get_text("name");
        let location = parameters.get_text("location");

        let result = CreateWorkspaceOperation {
            storage_provider: &self.storage,
        }
        .execute(CreateWorkspaceParameters {
            name: name.to_string(),
            location: Some(location.to_string()),
        });

        self.operation.set_result(result);
    }
}

#[test]
fn it_creates_workspace() {
    let mut test_case = CreateWorkspaceTestCase::default();

    test_case.execute_operation(table! {
        name = "Ironman"
        location = "/home/ironman"
    });

    test_case.inspect_operation_results(table! {
        [storage.workspace]
        name = "Ironman"
        location = "/home/ironman"
    });
}
