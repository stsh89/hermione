use std::{collections::HashMap, sync::RwLock};

#[derive(Clone)]
pub struct MockNotionCommandEntry {
    pub external_id: String,
    pub name: String,
    pub program: String,
    pub workspace_id: String,
}

pub struct MockNotionStorage {
    pub api_key: String,
    pub commands_database_id: String,
    pub workspaces_database_id: String,
    pub commands: RwLock<HashMap<String, MockNotionCommandEntry>>,
    pub workspaces: RwLock<HashMap<String, MockNotionWorkspaceEntry>>,
}

#[derive(Clone)]
pub struct MockNotionWorkspaceEntry {
    pub external_id: String,
    pub name: String,
    pub location: String,
}

impl Default for MockNotionStorage {
    fn default() -> Self {
        MockNotionStorage {
            api_key: "test_api_key".to_string(),
            commands_database_id: "test_commands_database_id".to_string(),
            workspaces_database_id: "test_workspaces_database_id".to_string(),
            commands: Default::default(),
            workspaces: Default::default(),
        }
    }
}
