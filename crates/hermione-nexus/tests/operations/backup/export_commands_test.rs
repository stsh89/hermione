use crate::{
    prelude::*,
    support::{self, InMemoryStorage, MockNotionBuilder, MockNotionStorage},
};
use hermione_nexus::{
    definitions::BackupProviderKind,
    operations::{ExportCommandsOperation, ExportCommandsOperationParameters},
};
use std::rc::Rc;

#[derive(Default)]
struct ExportCommandsTestCase {
    storage: InMemoryStorage,
    notion: Rc<MockNotionStorage>,
    operation: Operation<()>,
}

impl TestCase for ExportCommandsTestCase {
    fn execute_operation(&mut self, parameters: Table) {
        let backup_provider_name = parameters.get_text("backup_provider_kind");
        let backup_provider_kind = support::get_backup_provider_kind(backup_provider_name);

        let backup_builder = match backup_provider_kind {
            BackupProviderKind::Notion => &MockNotionBuilder {
                storage: self.notion.clone(),
            },
        };

        let result = ExportCommandsOperation::new(ExportCommandsOperationParameters {
            backup_credentials: &self.storage,
            commands: &self.storage,
            backup_builder,
        })
        .execute(backup_provider_kind);

        self.operation.set_result(result);
    }

    fn inspect_operation_results(&self, expectations: Table) {
        self.operation.assert_is_success();

        let backup_table = expectations.get_table("backup");
        let backup_provider_name = backup_table.get_text("provider_kind");
        let backup_provider_kind = support::get_backup_provider_kind(backup_provider_name);
        let command_table = backup_table.get_table("command");

        match backup_provider_kind {
            BackupProviderKind::Notion => {
                let external_id = command_table.get_text("external_id");
                let notion_command = support::get_notion_command(&self.notion, external_id);

                support::assert_notion_command(&notion_command, command_table);
            }
        }
    }

    fn setup(&mut self, parameters: Table) {
        let storage_table = parameters.get_table("storage");
        let credentials_table = &storage_table.get_table("backup_credentials");
        let workspace = storage_table.get_table("workspace");
        let command = storage_table.get_table("command");

        support::insert_workspace(&self.storage, workspace);
        support::insert_command(&self.storage, command);

        if let Some(credentials) = credentials_table.maybe_get_table("notion") {
            support::insert_notion_backup_credentials(&self.storage, credentials);
        }
    }
}

#[test]
fn it_sends_commands_to_notion() {
    let mut test_case = ExportCommandsTestCase::default();

    test_case.setup(table! {
        [storage.workspace]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        last_access_time = "2024-11-17 20:00:00"
        location = "/home/ironman"
        name = "Ironman"

        [storage.command]
        id = "51280bfc-2eea-444a-8df9-a1e7158c2c6b"
        name = "Ping"
        program = "ping 1.1.1.1"
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"

        [storage.backup_credentials.notion]
        api_key = "test_api_key"
        commands_database_id = "test_commands_database_id"
        workspaces_database_id = "test_workspaces_database_id"
    });

    test_case.execute_operation(table! {
        backup_provider_kind = "Notion"
    });

    test_case.inspect_operation_results(table! {
        [backup]
        provider_kind = "Notion"

        [backup.command]
        external_id = "51280bfc-2eea-444a-8df9-a1e7158c2c6b"
        name = "Ping"
        program = "ping 1.1.1.1"
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    });
}
