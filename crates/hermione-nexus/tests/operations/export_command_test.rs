use crate::{
    prelude::*,
    support::{self, InMemoryStorage, MockNotionBuilder, MockNotionStorage},
};
use hermione_nexus::{
    definitions::BackupProviderKind,
    operations::{
        ExportCommandOperation, ExportCommandOperationParameters, ExportCommandParameters,
    },
};
use std::rc::Rc;

#[derive(Default)]
struct ExportCommandToNotionTestCase {
    storage: InMemoryStorage,
    notion_storage: Rc<MockNotionStorage>,
    operation: Operation<()>,
}

impl TestCase for ExportCommandToNotionTestCase {
    fn setup(&mut self, parameters: Table) {
        let storage_table = parameters.get_table("storage");
        let credentials_table = &storage_table.get_table("backup_credentials");
        let command = storage_table.get_table("command");
        let notion_backup_credentials = credentials_table.get_table("notion");

        support::insert_command(&self.storage, command);
        support::insert_notion_backup_credentials(&self.storage, notion_backup_credentials);
    }

    fn execute_operation(&mut self, parameters: Table) {
        let command_id = parameters.get_uuid("command_id");
        let backup_provider_name = parameters.get_text("backup_provider_kind");
        let backup_provider_kind = support::get_backup_provider_kind(backup_provider_name);

        let result = ExportCommandOperation::new(ExportCommandOperationParameters {
            find_backup_credentials: &self.storage,
            find_command: &self.storage,
            backup_provider_builder: &MockNotionBuilder {
                storage: self.notion_storage.clone(),
            },
        })
        .execute(ExportCommandParameters {
            command_id: command_id.into(),
            backup_provider_kind,
        });

        self.operation.set_result(result);
    }

    fn assert_operation_success(&self, parameters: Table) {
        self.operation.assert_success();

        let backup_table = parameters.get_table("backup");
        let backup_provider_name = backup_table.get_text("provider_kind");
        let backup_provider_kind = support::get_backup_provider_kind(backup_provider_name);

        match backup_provider_kind {
            BackupProviderKind::Notion => {
                let command_table = backup_table.get_table("command");
                let external_id = command_table.get_text("external_id");
                let notion_command = support::get_notion_command(&self.notion_storage, external_id);

                support::assert_notion_command(&notion_command, command_table);
            }
        }
    }
}

#[test]
fn it_saves_workspace_into_notion_database() {
    let mut test_case = ExportCommandToNotionTestCase::default();

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
        command_id = "51280bfc-2eea-444a-8df9-a1e7158c2c6b"
    });

    test_case.assert_operation_success(table! {
        [backup]
        provider_kind = "Notion"

        [backup.command]
        external_id = "51280bfc-2eea-444a-8df9-a1e7158c2c6b"
        name = "Ping"
        program = "ping 1.1.1.1"
        workspace_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    });
}
