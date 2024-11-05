use hermione_nexus::{
    definitions::{BackupCredentials, Command, NotionBackupCredentials, Workspace},
    services::{
        BackupProvider, BackupProviderBuilder, ListCommandsBackup, ListWorkspacesBackup,
        UpsertCommandsBackup, UpsertWorkspacesBackup, VerifyBackupCredentials,
    },
    Error,
};
use std::{
    collections::HashMap,
    rc::Rc,
    sync::{PoisonError, RwLock},
};
use uuid::Uuid;

pub const TEST_NOTION_API_KEY: &str = "test_notion_api_key";

#[derive(thiserror::Error, Debug)]
pub enum MockBackupError {
    #[error("Memory access error: {0}")]
    MemoryAccess(String),
}

pub struct MockBackupBuilder {
    commands: Rc<RwLock<HashMap<Uuid, Command>>>,
    workspaces: Rc<RwLock<HashMap<Uuid, Workspace>>>,
}

pub struct MockBackup {
    credentials: NotionBackupCredentials,
    commands: Rc<RwLock<HashMap<Uuid, Command>>>,
    workspaces: Rc<RwLock<HashMap<Uuid, Workspace>>>,
}

#[derive(Default)]
pub struct MockStorageBackup {
    commands: Rc<RwLock<HashMap<Uuid, Command>>>,
    workspaces: Rc<RwLock<HashMap<Uuid, Workspace>>>,
}

pub struct MockBackupParameters {
    pub credentials: NotionBackupCredentials,
    pub commands: Rc<RwLock<HashMap<Uuid, Command>>>,
    pub workspaces: Rc<RwLock<HashMap<Uuid, Workspace>>>,
}

impl MockBackup {
    pub fn commands(&self) -> Result<Vec<Command>, MockBackupError> {
        let mut commands: Vec<Command> = self.commands.read()?.values().cloned().collect();

        commands.sort_by(|a, b| a.program().cmp(b.program()));

        Ok(commands)
    }

    pub fn count_commands(&self) -> Result<usize, MockBackupError> {
        let count = self.commands.read()?.len();

        Ok(count)
    }

    pub fn count_workspaces(&self) -> Result<usize, MockBackupError> {
        let count = self.workspaces.read()?.len();

        Ok(count)
    }

    pub fn insert_command(&self, command: &Command) -> Result<(), MockBackupError> {
        let mut commands = self.commands.write()?;

        commands.insert(**command.id(), command.clone());

        Ok(())
    }

    pub fn insert_workspace(&self, workspace: &Workspace) -> Result<(), MockBackupError> {
        let mut workspaces = self.workspaces.write()?;

        workspaces.insert(**workspace.id(), workspace.clone());

        Ok(())
    }

    pub fn new(parameters: MockBackupParameters) -> Self {
        let MockBackupParameters {
            credentials,
            commands,
            workspaces,
        } = parameters;

        Self {
            credentials,
            commands,
            workspaces,
        }
    }

    pub fn workspaces(&self) -> Result<Vec<Workspace>, MockBackupError> {
        let mut workspaces: Vec<Workspace> = self.workspaces.read()?.values().cloned().collect();

        workspaces.sort_by(|a, b| a.name().cmp(b.name()));

        Ok(workspaces)
    }
}

impl MockBackupBuilder {
    pub fn build(&self, credentials: BackupCredentials) -> MockBackup {
        let BackupCredentials::Notion(credentials) = credentials;

        MockBackup::new(MockBackupParameters {
            credentials,
            commands: self.commands.clone(),
            workspaces: self.workspaces.clone(),
        })
    }

    pub fn new(
        commands: Rc<RwLock<HashMap<Uuid, Command>>>,
        workspaces: Rc<RwLock<HashMap<Uuid, Workspace>>>,
    ) -> Self {
        Self {
            commands,
            workspaces,
        }
    }
}

impl MockStorageBackup {
    pub fn commands(&self) -> Rc<RwLock<HashMap<Uuid, Command>>> {
        self.commands.clone()
    }

    pub fn workspaces(&self) -> Rc<RwLock<HashMap<Uuid, Workspace>>> {
        self.workspaces.clone()
    }
}

impl<T> From<PoisonError<T>> for MockBackupError {
    fn from(err: PoisonError<T>) -> Self {
        Self::MemoryAccess(err.to_string())
    }
}

impl From<MockBackupError> for Error {
    fn from(err: MockBackupError) -> Self {
        Self::Storage(eyre::Error::new(err))
    }
}

fn index_from_str(page_id: Option<&str>) -> Result<usize, Error> {
    let page_id = page_id
        .map(|id| id.parse())
        .transpose()
        .map_err(|err| Error::Backup(format!("Invalid page ID: {}", err)))?
        .unwrap_or(0);

    Ok(page_id)
}

fn get_page<T>(collection: Vec<T>, index: usize) -> Vec<T> {
    let page_size = 10;

    collection
        .into_iter()
        .skip(index * page_size)
        .take(page_size)
        .collect()
}

impl BackupProviderBuilder<MockBackup> for MockBackupBuilder {
    fn build_backup_provider(&self, credentials: &BackupCredentials) -> Result<MockBackup, Error> {
        Ok(self.build(credentials.clone()))
    }
}

impl BackupProvider for MockBackup {}

impl ListCommandsBackup for MockBackup {
    fn list_commands_backup(
        &self,
        page_id: Option<&str>,
    ) -> Result<Option<(Vec<Command>, Option<String>)>, Error> {
        let page_id: usize = index_from_str(page_id)?;

        let commands: Vec<Command> = self.commands()?;
        let commands = get_page(commands, page_id);

        if commands.is_empty() {
            return Ok(None);
        }

        Ok(Some((commands, Some((page_id + 1).to_string()))))
    }
}

impl ListWorkspacesBackup for MockBackup {
    fn list_workspaces_backup(
        &self,
        page_id: Option<&str>,
    ) -> Result<Option<(Vec<Workspace>, Option<String>)>, Error> {
        let page_id = index_from_str(page_id)?;

        let workspaces: Vec<Workspace> = self.workspaces()?;
        let workspaces = get_page(workspaces, page_id);

        if workspaces.is_empty() {
            return Ok(None);
        }

        Ok(Some((workspaces, Some((page_id + 1).to_string()))))
    }
}

impl UpsertCommandsBackup for MockBackup {
    fn upsert_commands_backup(&self, commands: Vec<Command>) -> hermione_nexus::Result<()> {
        for command in commands {
            self.insert_command(&command)?;
        }

        Ok(())
    }
}

impl UpsertWorkspacesBackup for MockBackup {
    fn upsert_workspaces_backup(&self, workspaces: Vec<Workspace>) -> hermione_nexus::Result<()> {
        for workspace in workspaces {
            self.insert_workspace(&workspace)?;
        }

        Ok(())
    }
}

impl VerifyBackupCredentials for MockBackup {
    fn verify_backup_credentials(&self) -> Result<bool, Error> {
        Ok(self.credentials.api_key() == TEST_NOTION_API_KEY)
    }
}
