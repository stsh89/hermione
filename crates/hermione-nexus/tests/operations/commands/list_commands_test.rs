use crate::{
    prelude::*,
    support::{self, InMemoryStorage},
};
use hermione_nexus::{
    definitions::Command,
    operations::{ListCommandsOperation, ListCommandsParameters},
};
use std::num::NonZeroU32;

#[derive(Default)]
struct ListCommandsTestCase {
    storage: InMemoryStorage,
    operation: Operation<Vec<Command>>,
}

impl TestCase for ListCommandsTestCase {
    fn inspect_operation_results(&self, parameters: Table) {
        self.operation.assert_is_success();

        let response_table = parameters.get_table("response");
        let commands_table = response_table.get_array("commands");
        let commands = self.operation.result().as_ref().unwrap();

        assert_eq!(commands_table.len(), commands.len());

        for (index, command_table) in commands_table.into_iter().enumerate() {
            let command = commands.get(index).unwrap_or_else(|| {
                panic!(
                    "Total number of commands is {}, but index is {}",
                    commands.len(),
                    index
                )
            });

            support::assert_command(command, &command_table);
        }
    }

    fn execute_operation(&mut self, parameters: Table) {
        let program_contains = parameters.maybe_get_text("program_contains");
        let page_number = parameters.maybe_get_integer("page_number");
        let page_size = parameters.maybe_get_integer("page_size");
        let workspace_id = parameters.maybe_get_workspace_id("workspace_id");

        let result = ListCommandsOperation {
            provider: &self.storage,
        }
        .execute(ListCommandsParameters {
            program_contains,
            workspace_id,
            page_number: page_number.map(|v| unsafe { NonZeroU32::new_unchecked(v as u32) }),
            page_size: page_size.map(|v| unsafe { NonZeroU32::new_unchecked(v as u32) }),
        });

        self.operation.set_result(result);
    }

    fn setup(&mut self, parameters: Table) {
        let storage_table = parameters.get_table("storage");

        support::insert_workspace(&self.storage, storage_table.get_table("workspace"));

        for command in storage_table.get_array("commands") {
            support::insert_command(&self.storage, command);
        }
    }
}

#[test]
fn it_returns_commands_filtered_by_program() {
    let mut test_case = ListCommandsTestCase::default();

    test_case.setup(table! {
        [storage.workspace]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        location = "/home/ironman"
        name = "Ironman"

        [[storage.commands]]
        id = "51280bfc-2eea-444a-8df9-a1e7158c2c6b"
        name = "Ping"
        program = "ping 1.1.1.1"
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"

        [[storage.commands]]
        id = "657acc69-aafe-426d-8496-9859bc40ca62"
        name = "Get directory items"
        program = "GetChild-Item ."
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    });

    test_case.execute_operation(table! {
        program_contains = "Item"
    });

    test_case.inspect_operation_results(table! {
        [[response.commands]]
        id = "657acc69-aafe-426d-8496-9859bc40ca62"
        name = "Get directory items"
        program = "GetChild-Item ."
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    });
}

#[test]
fn it_returns_commands_by_page_number_and_page_size() {
    let mut test_case = ListCommandsTestCase::default();

    test_case.setup(table! {
        [storage.workspace]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        location = "/home/ironman"
        name = "Ironman"

        [[storage.commands]]
        id = "51280bfc-2eea-444a-8df9-a1e7158c2c6b"
        name = "Ping"
        program = "ping 1.1.1.1"
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"

        [[storage.commands]]
        id = "657acc69-aafe-426d-8496-9859bc40ca62"
        name = "Get directory items"
        program = "getchild-Item ."
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"

        [[storage.commands]]
        id = "1d0c6b79-2ea9-4291-85bf-84b9412c3a52"
        name = "Generate new UUID"
        program = "new-guid"
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        last_execute_time = "2024-11-17 11:00:00"

        [[storage.commands]]
        id = "12fe0231-2850-4f9b-b11c-844147f50b3d"
        name = "Lint Rust codebase"
        program = "becon"
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    });

    test_case.execute_operation(table! {
        page_number = 2
        page_size = 2
    });

    test_case.inspect_operation_results(table! {
        [[response.commands]]
        id = "657acc69-aafe-426d-8496-9859bc40ca62"
        name = "Get directory items"
        program = "getchild-Item ."
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"

        [[response.commands]]
        id = "51280bfc-2eea-444a-8df9-a1e7158c2c6b"
        name = "Ping"
        program = "ping 1.1.1.1"
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    });
}

#[test]
fn it_returns_commands_sorted_by_last_execute_time_and_program() {
    let mut test_case = ListCommandsTestCase::default();

    test_case.setup(table! {
        [storage.workspace]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        location = "/home/ironman"
        name = "Ironman"

        [[storage.commands]]
        id = "51280bfc-2eea-444a-8df9-a1e7158c2c6b"
        name = "Ping"
        program = "ping 1.1.1.1"
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"

        [[storage.commands]]
        id = "657acc69-aafe-426d-8496-9859bc40ca62"
        name = "Get directory items"
        program = "getchild-Item ."
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"

        [[storage.commands]]
        id = "1d0c6b79-2ea9-4291-85bf-84b9412c3a52"
        name = "Generate new UUID"
        program = "new-guid"
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        last_execute_time = "2024-11-17 11:00:00"
    });

    test_case.execute_operation(table! {
        page_number = 1
        page_size = 10
    });

    test_case.inspect_operation_results(table! {
        [[response.commands]]
        id = "1d0c6b79-2ea9-4291-85bf-84b9412c3a52"
        name = "Generate new UUID"
        program = "new-guid"
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        last_execute_time = "2024-11-17 11:00:00"

        [[response.commands]]
        id = "657acc69-aafe-426d-8496-9859bc40ca62"
        name = "Get directory items"
        program = "getchild-Item ."
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"

        [[response.commands]]
        id = "51280bfc-2eea-444a-8df9-a1e7158c2c6b"
        name = "Ping"
        program = "ping 1.1.1.1"
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    });
}
