use crate::support::{self, InMemoryStorage, MockNotionBackupBuilder, MockNotionStorage};
use anyhow::Result;
use chrono::NaiveDateTime;
use hermione_nexus::{
    definitions::BackupProviderKind,
    operations::{ImportWorkspacesOperation, ImportWorkspacesOperationParameters},
};
use serde_json::json;
use serde_json::Value as Json;
use std::rc::Rc;

struct TestContext {
    storage: InMemoryStorage,
    notion_storage: Rc<MockNotionStorage>,
}

impl TestContext {
    fn assert_storage_contains_workspace(&self, parameters: Json) {
        let id = parameters["id"].as_str().unwrap().parse().unwrap();

        let last_access_time = parameters["last_access_time"].as_str().map(|value| {
            NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .and_utc()
        });

        let name = parameters["name"].as_str().unwrap_or_default();
        let location = parameters["location"].as_str();

        let workspace = support::get_workspace(&self.storage, id);

        assert_eq!(workspace.name(), name);
        assert_eq!(workspace.location(), location);
        assert_eq!(workspace.last_access_time(), last_access_time.as_ref());
    }

    fn import_workspaces_to_notion(&self) -> Result<()> {
        ImportWorkspacesOperation::new(ImportWorkspacesOperationParameters {
            backup_credentials_provider: &self.storage,
            upsert_workspaces_provider: &self.storage,
            backup_provider_builder: &MockNotionBackupBuilder {
                storage: self.notion_storage.clone(),
            },
        })
        .execute(BackupProviderKind::Notion)?;

        Ok(())
    }

    fn with_background() -> Self {
        let ctx = TestContext {
            storage: InMemoryStorage::empty(),
            notion_storage: Rc::new(MockNotionStorage::empty()),
        };

        notion_storage_contains_workspace_entry(
            &ctx,
            json!({
                "external_id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                "name": "Ironman",
                "location": "/home/ironman"
            }),
        );

        storage_contains_valid_backup_credentials(
            &ctx,
            json!({
                "api_key": "test_api_key",
                "commands_database_id": "test_commands_database_id",
                "workspaces_database_id": "test_workspaces_database_id"
            }),
        );

        ctx
    }
}

fn storage_contains_valid_backup_credentials(ctx: &TestContext, parameters: Json) {
    support::insert_notion_backup_credentials(&ctx.storage, parameters)
}

fn storage_contains_workspace(ctx: &TestContext, parameters: Json) {
    support::insert_workspace(&ctx.storage, parameters);
}

fn notion_storage_contains_workspace_entry(ctx: &TestContext, parameters: Json) {
    support::insert_notion_workspace(&ctx.notion_storage, parameters);
}

#[test]
fn it_restores_workspaces() -> Result<()> {
    let context = TestContext::with_background();

    context.import_workspaces_to_notion()?;

    context.assert_storage_contains_workspace(json!({
        "id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
        "name": "Ironman",
        "location": "/home/ironman",
        "last_access_time": null
    }));

    Ok(())
}

#[test]
fn it_updates_existing_workspaces() -> Result<()> {
    let context = TestContext::with_background();

    storage_contains_workspace(
        &context,
        json!({
            "id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
            "name": "Avenger",
            "location": "/home/avenger",
            "last_access_time": "2024-11-17 20:00:00"
        }),
    );

    context.import_workspaces_to_notion()?;

    context.assert_storage_contains_workspace(json!({
        "id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
        "name": "Ironman",
        "location": "/home/ironman",
        "last_access_time": null
    }));

    Ok(())
}
