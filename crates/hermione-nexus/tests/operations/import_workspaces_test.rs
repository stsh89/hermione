use crate::support::{
    self, InMemoryStorage, MockNotionBackupBuilder, MockNotionStorage, MockNotionWorkspaceEntry,
};
use anyhow::Result;
use hermione_nexus::{
    definitions::{BackupCredentials, BackupProviderKind, NotionBackupCredentialsParameters},
    operations::{ImportWorkspacesOperation, ImportWorkspacesOperationParameters},
};
use std::rc::Rc;

struct ImportWorkspacesFromNotionBackground {
    storage: InMemoryStorage,
    notion_storage: MockNotionStorage,
}

fn with_notion_background<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(ImportWorkspacesFromNotionBackground) -> Result<()>,
{
    let storage = support::prepare_storage(|storage| {
        support::insert_backup_credentials(
            storage,
            BackupCredentials::notion(NotionBackupCredentialsParameters {
                api_key: "test_api_key".to_string(),
                commands_database_id: "test_commands_database_id".to_string(),
                workspaces_database_id: "test_workspaces_database_id".to_string(),
            }),
        )
    });

    let notion_storage = support::prepare_notion_storage(|storage| {
        support::insert_notion_workspace(
            storage,
            MockNotionWorkspaceEntry {
                external_id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa".to_string(),
                name: "Ironman".to_string(),
                location: "/home/ironman".to_string(),
            },
        );
    });

    test_fn(ImportWorkspacesFromNotionBackground {
        notion_storage,
        storage,
    })
}

#[test]
fn it_restores_workspaces_from_notion() -> Result<()> {
    with_notion_background(|ctx| {
        let ImportWorkspacesFromNotionBackground {
            storage,
            notion_storage,
        } = ctx;

        assert_eq!(support::count_notion_workspaces(&notion_storage), 1);
        assert_eq!(support::count_workspaces(&storage), 0);

        ImportWorkspacesOperation::new(ImportWorkspacesOperationParameters {
            backup_credentials_provider: &storage,
            upsert_workspaces_provider: &storage,
            backup_provider_builder: &MockNotionBackupBuilder {
                storage: Rc::new(notion_storage),
            },
        })
        .execute(BackupProviderKind::Notion)?;

        assert_eq!(storage.count_workspaces()?, 1);

        let workspace =
            support::get_workspace(&storage, "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa".parse()?);

        assert_eq!(workspace.name(), "Ironman");
        assert_eq!(workspace.location(), Some("/home/ironman"));

        Ok(())
    })
}
