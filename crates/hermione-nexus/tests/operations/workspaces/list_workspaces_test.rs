use std::num::NonZeroU32;

use crate::{
    prelude::*,
    support::{self, InMemoryStorage},
};
use hermione_nexus::{
    definitions::Workspace,
    operations::{ListWorkspacesOperation, ListWorkspacesParameters},
};

#[derive(Default)]
struct ListWorkspacesTestCase {
    storage: InMemoryStorage,
    operation: Operation<Vec<Workspace>>,
}

impl TestCase for ListWorkspacesTestCase {
    fn inspect_operation_results(&self, parameters: Table) {
        self.operation.assert_is_success();

        let response_table = parameters.get_table("response");
        let workspaces_table = response_table.get_array("workspaces");
        let workspaces = self.operation.result().as_ref().unwrap();

        assert_eq!(workspaces_table.len(), workspaces.len());

        for (index, workspace_table) in workspaces_table.iter().enumerate() {
            let workspace = workspaces.get(index).unwrap_or_else(|| {
                panic!(
                    "Total number of workspaces is {}, but index is {}",
                    workspaces.len(),
                    index
                )
            });

            support::assert_workspace(workspace, workspace_table);
        }
    }

    fn execute_operation(&mut self, parameters: Table) {
        let name_contains = parameters.maybe_get_text("name_contains");
        let page_number = parameters.maybe_get_integer("page_number");
        let page_size = parameters.maybe_get_integer("page_size");

        let result = ListWorkspacesOperation {
            provider: &self.storage,
        }
        .execute(ListWorkspacesParameters {
            name_contains,
            page_number: page_number.map(|v| unsafe { NonZeroU32::new_unchecked(v as u32) }),
            page_size: page_size.map(|v| unsafe { NonZeroU32::new_unchecked(v as u32) }),
        });

        self.operation.set_result(result);
    }

    fn setup(&mut self, parameters: Table) {
        let storage_table = parameters.get_table("storage");

        for workspace in storage_table.get_array("workspaces") {
            support::insert_workspace(&self.storage, workspace);
        }
    }
}

#[test]
fn it_returns_workspaces_filtered_by_name() {
    let mut test_case = ListWorkspacesTestCase::default();

    test_case.setup(table! {
        [[storage.workspaces]]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        location = "/home/ironman"
        name = "Ironman"

        [[storage.workspaces]]
        id = "637d207c-7a18-47eb-b0b4-7f27d4ecbf88"
        location = "/home/avenger"
        name = "Avenger"
    });

    test_case.execute_operation(table! {
        name_contains = "man"
    });

    test_case.inspect_operation_results(table! {
        [[response.workspaces]]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        location = "/home/ironman"
        name = "Ironman"
    });
}

#[test]
fn it_returns_workspaces_by_page_number_and_page_size() {
    let mut test_case = ListWorkspacesTestCase::default();

    test_case.setup(table! {
        [[storage.workspaces]]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        location = "/home/ironman"
        name = "Ironman"

        [[storage.workspaces]]
        id = "637d207c-7a18-47eb-b0b4-7f27d4ecbf88"
        location = "/home/avenger"
        name = "Avenger"

        [[storage.workspaces]]
        id = "19e0f51f-efaa-4b22-a35d-17c37f350823"
        location = "/home/batman"
        name = "Batman"
        last_access_time = "2024-11-17 20:00:00"

        [[storage.workspaces]]
        id = "d9469304-ec44-4c84-8612-7ba3c27b9e29"
        location = "/home/vision"
        name = "Vision"
    });

    test_case.execute_operation(table! {
        page_number = 2
        page_size = 2
    });

    test_case.inspect_operation_results(table! {
        [[response.workspaces]]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        location = "/home/ironman"
        name = "Ironman"

        [[response.workspaces]]
        id = "d9469304-ec44-4c84-8612-7ba3c27b9e29"
        location = "/home/vision"
        name = "Vision"
    });
}

#[test]
fn it_returns_workspaces_sorted_by_last_access_time_and_name() {
    let mut test_case = ListWorkspacesTestCase::default();

    test_case.setup(table! {
        [[storage.workspaces]]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        location = "/home/ironman"
        name = "Ironman"

        [[storage.workspaces]]
        id = "637d207c-7a18-47eb-b0b4-7f27d4ecbf88"
        location = "/home/avenger"
        name = "Avenger"

        [[storage.workspaces]]
        id = "19e0f51f-efaa-4b22-a35d-17c37f350823"
        location = "/home/batman"
        name = "Batman"
        last_access_time = "2024-11-17 20:00:00"
    });

    test_case.execute_operation(table! {
        page_number = 1
        page_size = 10
    });

    test_case.inspect_operation_results(table! {
        [[response.workspaces]]
        id = "19e0f51f-efaa-4b22-a35d-17c37f350823"
        location = "/home/batman"
        name = "Batman"
        last_access_time = "2024-11-17 20:00:00"

        [[response.workspaces]]
        id = "637d207c-7a18-47eb-b0b4-7f27d4ecbf88"
        location = "/home/avenger"
        name = "Avenger"

        [[response.workspaces]]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        location = "/home/ironman"
        name = "Ironman"
    });
}
