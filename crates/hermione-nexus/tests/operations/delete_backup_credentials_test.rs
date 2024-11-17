use crate::support::{self, InMemoryStorage};
use anyhow::Result;
use hermione_nexus::{
    definitions::BackupProviderKind, operations::DeleteBackupCredentialsOperation, Error,
};
use serde_json::json;

#[derive(Default)]
struct DeleteBackupCredentialsTestContext {
    storage: InMemoryStorage,
    error: Option<Error>,
}

impl DeleteBackupCredentialsTestContext {
    fn assert_error_contains_message(&self, message: &str) {
        assert_eq!(self.error.as_ref().unwrap().to_string(), message);
    }

    fn assert_storage_backup_credentials_empty(&self) {
        assert!(support::list_backup_credentials(&self.storage).is_empty());
    }

    fn delete_notion_backup_credentials(&self) -> Result<()> {
        self.execute_operation(BackupProviderKind::Notion)?;

        Ok(())
    }

    fn execute_operation(
        &self,
        backup_provider_kind: BackupProviderKind,
    ) -> hermione_nexus::Result<()> {
        DeleteBackupCredentialsOperation {
            find_provider: &self.storage,
            delete_provider: &self.storage,
        }
        .execute(&backup_provider_kind)
    }

    fn try_to_delete_notion_backup_credentials(&mut self) -> Result<()> {
        let error = self
            .execute_operation(BackupProviderKind::Notion)
            .unwrap_err();

        self.error = Some(error);

        Ok(())
    }

    fn with_notion_background() -> Self {
        let context = Self::default();

        support::insert_notion_backup_credentials(
            &context.storage,
            json!({
                "api_key": "test_api_key",
                "commands_database_id": "test_commands_database_id",
                "workspaces_database_id": "test_workspaces_database_id",
            }),
        );

        context
    }
}

#[test]
fn it_deletes_backup_credentials() -> Result<()> {
    let context = DeleteBackupCredentialsTestContext::with_notion_background();

    context.delete_notion_backup_credentials()?;
    context.assert_storage_backup_credentials_empty();

    Ok(())
}

#[test]
fn it_returns_not_found_error() -> Result<()> {
    let mut context = DeleteBackupCredentialsTestContext::default();

    context.try_to_delete_notion_backup_credentials()?;
    context.assert_error_contains_message("Backup credentials not found");

    Ok(())
}
