mod backup;
mod clipboard;
mod fixtures;
mod notion_storage;
mod storage;
mod system;

use hermione_nexus::definitions::{BackupCredentials, Workspace};

pub use backup::*;
pub use clipboard::*;
pub use fixtures::*;
pub use notion_storage::*;
pub use storage::*;
pub use system::*;

use uuid::Uuid;

pub fn count_notion_workspaces(storage: &MockNotionStorage) -> usize {
    let workspaces = storage
        .workspaces
        .read()
        .expect("Should be able to count Notion workspaces");

    workspaces.len()
}

pub fn count_workspaces(storage: &InMemoryStorage) -> usize {
    let workspaces = storage
        .workspaces
        .read()
        .expect("Should be able to count workspaces");

    workspaces.len()
}

pub fn get_workspace(storage: &InMemoryStorage, id: Uuid) -> Workspace {
    storage
        .workspaces
        .read()
        .expect("Should be able to get workspace")
        .get(&id)
        .cloned()
        .unwrap_or_else(|| panic!("Workspace {} should exist", id.braced()))
}

pub fn insert_backup_credentials(storage: &InMemoryStorage, credentials: BackupCredentials) {
    let id = match credentials {
        BackupCredentials::Notion(_) => NOTION_CREDENTIALS_KEY.to_string(),
    };

    storage
        .backup_credentials
        .write()
        .expect("Should be able to insert backup credentials")
        .insert(id, credentials);
}

pub fn insert_workspace(storage: &InMemoryStorage, workspace: Workspace) {
    storage
        .workspaces
        .write()
        .expect("Should be able to insert workspace")
        .insert(**workspace.id(), workspace);
}

pub fn insert_notion_workspace(storage: &MockNotionStorage, workspace: MockNotionWorkspaceEntry) {
    storage
        .workspaces
        .write()
        .expect("Should be able to insert Notion workspace")
        .insert(workspace.external_id.clone(), workspace);
}

pub fn prepare_notion_storage<T>(update_storage: T) -> MockNotionStorage
where
    T: FnOnce(&MockNotionStorage),
{
    let storage = MockNotionStorage::default();

    update_storage(&storage);

    storage
}

pub fn prepare_storage<T>(update_storage: T) -> InMemoryStorage
where
    T: FnOnce(&InMemoryStorage),
{
    let storage = InMemoryStorage::default();

    update_storage(&storage);

    storage
}
