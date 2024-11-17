use crate::support::{self, InMemoryStorage, MockNotionBuilder, MockNotionStorage};
use anyhow::Result;
use hermione_nexus::{
    definitions::{BackupCredentials, NotionBackupCredentialsParameters},
    operations::SaveBackupCredentialsOperation,
    Error,
};
use serde_json::{json, Value as Json};
use std::{marker::PhantomData, rc::Rc};

#[derive(Default)]
struct SaveBackupCredentialsTestContext {
    storage: InMemoryStorage,
    notion_storage: Rc<MockNotionStorage>,
    error: Option<Error>,
}

impl SaveBackupCredentialsTestContext {
    fn assert_storage_backup_credentials_empty(&self) {
        assert!(support::list_backup_credentials(&self.storage).is_empty());
    }

    fn assert_error_contains_message(&self, message: &str) {
        assert_eq!(self.error.as_ref().unwrap().to_string(), message);
    }

    fn assert_storage_contains_notion_backup_credentials(&self, parameters: Json) {
        let backup_credentials = support::get_notion_backup_credentials(&self.storage);

        support::assert_notion_backup_credentials(&backup_credentials, parameters);
    }

    fn execute_operation(&self, parameters: Json) -> hermione_nexus::Result<()> {
        let backup_provider_kind = parameters["backup_provider_kind"].as_str().unwrap();

        let backup_credentials = match backup_provider_kind {
            "Notion" => BackupCredentials::notion(notion_backup_credentials_parameters(parameters)),
            _ => unreachable!(),
        };

        SaveBackupCredentialsOperation {
            save_provider: &self.storage,
            backup_provider_builder: &MockNotionBuilder {
                storage: self.notion_storage.clone(),
            },
            backup_provider: PhantomData,
        }
        .execute(&backup_credentials)
    }

    fn save_notion_backup_credentials(&self, parameters: Json) -> Result<()> {
        let mut parameters = parameters;
        parameters["backup_provider_kind"] = "Notion".into();

        self.execute_operation(parameters)?;

        Ok(())
    }

    fn try_to_save_notion_backup_credentials(&mut self, parameters: Json) -> Result<()> {
        let mut parameters = parameters;
        parameters["backup_provider_kind"] = "Notion".into();

        let error = self.execute_operation(parameters).unwrap_err();

        self.error = Some(error);

        Ok(())
    }
}

fn notion_backup_credentials_parameters(json: Json) -> NotionBackupCredentialsParameters {
    let api_key = json["api_key"].as_str().unwrap().to_string();
    let commands_database_id = json["commands_database_id"].as_str().unwrap().to_string();
    let workspaces_database_id = json["workspaces_database_id"].as_str().unwrap().to_string();

    NotionBackupCredentialsParameters {
        api_key,
        commands_database_id,
        workspaces_database_id,
    }
}

#[test]
fn it_saves_notion_backup_credentials() -> Result<()> {
    let context = SaveBackupCredentialsTestContext::default();

    context.save_notion_backup_credentials(json!({
        "api_key": "test_api_key",
        "commands_database_id": "test_commands_database_id",
        "workspaces_database_id": "test_workspaces_database_id",
    }))?;

    context.assert_storage_contains_notion_backup_credentials(json!({
        "api_key": "test_api_key",
        "commands_database_id": "test_commands_database_id",
        "workspaces_database_id": "test_workspaces_database_id",
    }));

    Ok(())
}

#[test]
fn it_returns_notion_api_key_verification_error() -> Result<()> {
    let mut context = SaveBackupCredentialsTestContext::default();

    context.try_to_save_notion_backup_credentials(json!({
        "api_key": "fake_api_key",
        "commands_database_id": "test_commands_database_id",
        "workspaces_database_id": "test_workspaces_database_id",
    }))?;

    context.assert_storage_backup_credentials_empty();
    context.assert_error_contains_message("Backup failure: Invalid API key");

    Ok(())
}
