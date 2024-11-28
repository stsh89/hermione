use crate::{
    prelude::*,
    support::{self, InMemoryStorage},
};
use hermione_nexus::{definitions::BackupCredentials, operations::GetBackupCredentialsOperation};

#[derive(Default)]
struct GetBackupCredentialsTestCase {
    storage: InMemoryStorage,
    operation: Operation<BackupCredentials>,
}

impl TestCase for GetBackupCredentialsTestCase {
    fn execute_operation(&mut self, parameters: Table) {
        let backup_provider_name = parameters.get_text("backup_provider_kind");
        let backup_provider_kind = support::get_backup_provider_kind(backup_provider_name);

        let result = GetBackupCredentialsOperation {
            provider: &self.storage,
        }
        .execute(backup_provider_kind);

        self.operation.set_result(result);
    }

    fn inspect_operation_results(&self, expectations: Table) {
        self.operation.assert_is_success();

        let credentials_table = expectations
            .get_table("response")
            .get_table("backup_credentials");
        let backup_credentials = self.operation.result().as_ref().unwrap();

        if let Some(notion_backup_credentials) = credentials_table.maybe_get_table("notion") {
            support::assert_notion_backup_credentials(
                backup_credentials,
                notion_backup_credentials,
            );
        }
    }

    fn setup(&mut self, parameters: Table) {
        let storage_table = parameters.get_table("storage");
        let credentials_table = &storage_table.get_table("backup_credentials");

        if let Some(notion_table) = credentials_table.maybe_get_table("notion") {
            support::insert_notion_backup_credentials(&self.storage, notion_table);
        }
    }
}

#[test]
fn it_returns_backup_credentials() {
    let mut test_case = GetBackupCredentialsTestCase::default();

    test_case.setup(table! {
        [storage.backup_credentials.notion]
        api_key = "test_api_key"
        commands_database_id = "test_commands_database_id"
        workspaces_database_id = "test_workspaces_database_id"
    });

    test_case.execute_operation(table! {
        backup_provider_kind = "Notion"
    });

    test_case.inspect_operation_results(table! {
        [response.backup_credentials.notion]
        api_key = "test_api_key"
        commands_database_id = "test_commands_database_id"
        workspaces_database_id = "test_workspaces_database_id"
    });
}
