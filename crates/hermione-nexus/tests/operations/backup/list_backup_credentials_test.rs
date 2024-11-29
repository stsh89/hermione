use crate::{
    prelude::*,
    support::{self, InMemoryStorage},
};
use hermione_nexus::{definitions::BackupCredentials, operations::ListBackupCredentialsOperation};

#[derive(Default)]
struct ListBackupCredentialsTestCase {
    storage: InMemoryStorage,
    operation: Operation<Vec<BackupCredentials>>,
}

impl TestCase for ListBackupCredentialsTestCase {
    fn execute_operation(&mut self, _parameters: Table) {
        let result = ListBackupCredentialsOperation {
            provider: &self.storage,
        }
        .execute();

        self.operation.set_result(result);
    }

    fn inspect_operation_results(&self, expectations: Table) {
        self.operation.assert_is_success();

        let response_table = expectations.get_table("response");
        let credentials_table = response_table.get_table("backup_credentials");
        let notion_backup_credentials = credentials_table.get_table("notion");
        let backup_credentials = self.operation.result().as_ref().unwrap();

        support::assert_notion_backup_credentials(
            &backup_credentials[0],
            notion_backup_credentials,
        );
    }

    fn setup(&mut self, parameters: Table) {
        let storage_table = parameters.get_table("storage");
        let credentials_table = &storage_table.get_table("backup_credentials");
        let notion_table = credentials_table.get_table("notion");

        support::insert_notion_backup_credentials(&self.storage, notion_table);
    }
}

#[test]
fn it_returns_backup_credentials() {
    let mut test_case = ListBackupCredentialsTestCase::default();

    test_case.setup(table! {
        [storage.backup_credentials.notion]
        api_key = "test_api_key"
        commands_database_id = "test_commands_database_id"
        workspaces_database_id = "test_workspaces_database_id"
    });

    test_case.execute_operation(table! {
        [parameters]
    });

    test_case.inspect_operation_results(table! {
        [response.backup_credentials.notion]
        api_key = "test_api_key"
        commands_database_id = "test_commands_database_id"
        workspaces_database_id = "test_workspaces_database_id"
    });
}
