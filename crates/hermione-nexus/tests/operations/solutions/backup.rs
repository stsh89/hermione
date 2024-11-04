use hermione_nexus::{
    definitions::{BackupCredentials, Command, Workspace},
    services::{BackupProvider, BackupProviderBuilder, ListCommandsBackup, ListWorkspacesBackup},
    Error,
};
use std::{
    collections::HashMap,
    sync::{PoisonError, RwLock},
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum MockBackupProviderError {
    #[error("Lock access: {0}")]
    LockAccess(String),
}

pub struct MockBackupProviderBuilder {
    commands: Vec<Command>,
    workspaces: Vec<Workspace>,
}

pub struct MockBackupProvider {
    commands: RwLock<HashMap<Uuid, Command>>,
    workspaces: RwLock<HashMap<Uuid, Workspace>>,
}

impl MockBackupProvider {
    pub fn commands(&self) -> Result<Vec<Command>, MockBackupProviderError> {
        let mut commands: Vec<Command> = self.commands.read()?.values().cloned().collect();

        commands.sort_by(|a, b| a.program().cmp(&b.program()));

        Ok(commands)
    }

    pub fn commands_count(&self) -> Result<usize, MockBackupProviderError> {
        let count = self.commands.read()?.len();

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

    pub fn new() -> Self {
        Self {
            commands: RwLock::new(HashMap::new()),
            workspaces: RwLock::new(HashMap::new()),
        }
    }

    pub fn workspaces(&self) -> Result<Vec<Workspace>, MockBackupProviderError> {
        let mut workspaces: Vec<Workspace> = self.workspaces.read()?.values().cloned().collect();

        workspaces.sort_by(|a, b| a.name().cmp(&b.name()));

        Ok(workspaces)
    }

    pub fn workspaces_count(&self) -> Result<usize, MockBackupProviderError> {
        let count = self.workspaces.read()?.len();

        Ok(count)
    }
}

impl MockBackupProviderBuilder {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            workspaces: Vec::new(),
        }
    }

    pub fn build(&self) -> Result<MockBackupProvider, MockBackupProviderError> {
        let backup_provider = MockBackupProvider::new();

        for command in &self.commands {
            backup_provider.insert_command(&command)?;
        }

        for workspace in &self.workspaces {
            backup_provider.insert_workspace(&workspace)?;
        }

        Ok(backup_provider)
    }

    pub fn set_commands(&mut self, commands: Vec<Command>) {
        self.commands = commands;
    }

    pub fn set_workspaces(&mut self, workspaces: Vec<Workspace>) {
        self.workspaces = workspaces;
    }
}

impl BackupProviderBuilder<MockBackupProvider> for MockBackupProviderBuilder {
    fn build_backup_provider(
        &self,
        _credentials: &BackupCredentials,
    ) -> Result<MockBackupProvider, Error> {
        let backup_provider = self.build()?;

        Ok(backup_provider)
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
