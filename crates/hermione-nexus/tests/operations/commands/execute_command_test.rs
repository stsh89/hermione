use crate::{
    prelude::*,
    support::{self, InMemoryStorage, MockSystem},
};
use hermione_nexus::operations::ExecuteCommandOperation;

#[derive(Default)]
struct ExecuteCommandTestCase {
    system: MockSystem,
    operation: Operation<()>,
    storage: InMemoryStorage,
}

impl TestCase for ExecuteCommandTestCase {
    fn setup(&mut self, parameters: Table) {
        let storage_table = parameters.get_table("storage");
        let system_table = parameters.get_table("system");

        support::insert_workspace(&self.storage, storage_table.get_table("workspace"));
        support::insert_command(&self.storage, storage_table.get_table("command"));
        support::freeze_storage_time(&self.storage, system_table.get_date_time("time"));
    }

    fn execute_operation(&mut self, parameters: Table) {
        let command_id = parameters.get_uuid("command_id").into();

        let result = ExecuteCommandOperation {
            find_command_provider: &self.storage,
            find_workspace_provider: &self.storage,
            system_provider: &self.system,
            track_command_provider: &self.storage,
            track_workspace_provider: &self.storage,
        }
        .execute(&command_id);

        self.operation.set_result(result);
    }

    fn inspect_operation_results(&self, parameters: Table) {
        self.operation.assert_is_success();

        let system_table = parameters.get_table("system");
        let last_executed_program = system_table.get_text("last_executed_program");
        let last_visited_location = system_table.get_text("last_visited_location");

        support::assert_last_executed_program(&self.system, last_executed_program);
        support::assert_file_system_location(&self.system, last_visited_location);

        let storage_table = parameters.get_table("storage");
        let workspace_table = storage_table.get_table("workspace");
        let command_table = storage_table.get_table("command");
        let last_access_time = workspace_table.get_date_time("last_access_time");
        let last_execute_time = command_table.get_date_time("last_execute_time");
        let workspace = support::get_workspace(&self.storage, workspace_table.get_uuid("id"));
        let command = support::get_command(&self.storage, command_table.get_uuid("id"));

        assert_eq!(command.last_execute_time(), Some(&last_execute_time));
        assert_eq!(workspace.last_access_time(), Some(&last_access_time))
    }
}

#[test]
fn it_runs_program_and_tracks_time() {
    let mut test_case = ExecuteCommandTestCase::default();

    test_case.setup(table! {
        [system]
        time = "2024-11-17 20:20:00"

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
        command_id = "51280bfc-2eea-444a-8df9-a1e7158c2c6b"
    });

    test_case.inspect_operation_results(table! {
        [system]
        last_executed_program = "ping 1.1.1.1"
        last_visited_location = "/home/ironman"

        [storage.workspace]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        last_access_time = "2024-11-17 20:20:01"

        [storage.command]
        id = "51280bfc-2eea-444a-8df9-a1e7158c2c6b"
        last_execute_time = "2024-11-17 20:20:01"
    });
}
