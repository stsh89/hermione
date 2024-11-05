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
pub enum MockBackupProviderError {
    #[error("Lock access: {0}")]
    LockAccess(String),
}

#[derive(Default)]
pub struct MockBackupProviderBuilder {
    commands: Rc<RwLock<HashMap<Uuid, Command>>>,
    workspaces: Rc<RwLock<HashMap<Uuid, Workspace>>>,
}

pub struct MockBackupProvider {
    credentials: NotionBackupCredentials,
    commands: Rc<RwLock<HashMap<Uuid, Command>>>,
    workspaces: Rc<RwLock<HashMap<Uuid, Workspace>>>,
}

pub struct MockBackupProviderParameters {
    pub credentials: NotionBackupCredentials,
    pub commands: Rc<RwLock<HashMap<Uuid, Command>>>,
    pub workspaces: Rc<RwLock<HashMap<Uuid, Workspace>>>,
}

impl MockBackupProvider {
    pub fn commands(&self) -> Result<Vec<Command>, MockBackupProviderError> {
        let mut commands: Vec<Command> = self.commands.read()?.values().cloned().collect();

        commands.sort_by(|a, b| a.program().cmp(b.program()));

        Ok(commands)
    }

    pub fn count_commands(&self) -> Result<usize, MockBackupProviderError> {
        let count = self.commands.read()?.len();

        Ok(count)
    }

    pub fn count_workspaces(&self) -> Result<usize, MockBackupProviderError> {
        let count = self.workspaces.read()?.len();

        Ok(count)
    }

    pub fn insert_command(&self, command: &Command) -> Result<(), MockBackupProviderError> {
        let mut commands = self.commands.write()?;

        commands.insert(**command.id(), command.clone());

        Ok(())
    }

    pub fn insert_workspace(&self, workspace: &Workspace) -> Result<(), MockBackupProviderError> {
        let mut workspaces = self.workspaces.write()?;

        workspaces.insert(**workspace.id(), workspace.clone());

        Ok(())
    }

    pub fn new(parameters: MockBackupProviderParameters) -> Self {
        let MockBackupProviderParameters {
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

    pub fn workspaces(&self) -> Result<Vec<Workspace>, MockBackupProviderError> {
        let mut workspaces: Vec<Workspace> = self.workspaces.read()?.values().cloned().collect();

        workspaces.sort_by(|a, b| a.name().cmp(b.name()));

        Ok(workspaces)
    }
}

impl MockBackupProviderBuilder {
    pub fn build(&self, credentials: BackupCredentials) -> MockBackupProvider {
        let BackupCredentials::Notion(credentials) = credentials;

        MockBackupProvider::new(MockBackupProviderParameters {
            credentials,
            commands: self.commands.clone(),
            workspaces: self.workspaces.clone(),
        })
    }

    pub fn commands(&self) -> Rc<RwLock<HashMap<Uuid, Command>>> {
        self.commands.clone()
    }

    pub fn workspaces(&self) -> Rc<RwLock<HashMap<Uuid, Workspace>>> {
        self.workspaces.clone()
    }
}

impl<T> From<PoisonError<T>> for MockBackupProviderError {
    fn from(err: PoisonError<T>) -> Self {
        Self::LockAccess(err.to_string())
    }
}

impl From<MockBackupProviderError> for Error {
    fn from(err: MockBackupProviderError) -> Self {
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

impl BackupProviderBuilder<MockBackupProvider> for MockBackupProviderBuilder {
    fn build_backup_provider(
        &self,
        credentials: &BackupCredentials,
    ) -> Result<MockBackupProvider, Error> {
        Ok(self.build(credentials.clone()))
    }
}

impl BackupProvider for MockBackupProvider {}

impl ListCommandsBackup for MockBackupProvider {
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

impl ListWorkspacesBackup for MockBackupProvider {
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

impl UpsertCommandsBackup for MockBackupProvider {
    fn upsert_commands_backup(&self, commands: Vec<Command>) -> hermione_nexus::Result<()> {
        for command in commands {
            self.insert_command(&command)?;
        }

        Ok(())
    }
}

impl UpsertWorkspacesBackup for MockBackupProvider {
    fn upsert_workspaces_backup(&self, workspaces: Vec<Workspace>) -> hermione_nexus::Result<()> {
        for workspace in workspaces {
            self.insert_workspace(&workspace)?;
        }

        Ok(())
    }
}

impl VerifyBackupCredentials for MockBackupProvider {
    fn verify_backup_credentials(&self) -> Result<bool, Error> {
        Ok(self.credentials.api_key() == TEST_NOTION_API_KEY)
    }
}
