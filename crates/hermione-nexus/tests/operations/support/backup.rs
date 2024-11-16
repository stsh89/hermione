use std::rc::Rc;

use super::{MockNotionCommandEntry, MockNotionStorage, MockNotionWorkspaceEntry};
use eyre::eyre;
use hermione_nexus::{
    definitions::{
        BackupCredentials, Command, CommandParameters, NotionBackupCredentials, Workspace,
        WorkspaceParameters,
    },
    services::{
        BackupService, BackupServiceBuilder, ListCommandsBackup, ListWorkspacesBackup,
        UpsertCommandsBackup, UpsertWorkspacesBackup, VerifyBackupCredentials,
    },
    Error, Result,
};
use uuid::Uuid;

const DEFAULT_PAGE_SIZE: usize = 10;

pub struct MockNotionBackupBuilder {
    pub storage: Rc<MockNotionStorage>,
}

pub struct MockNotionBackup {
    pub storage: Rc<MockNotionStorage>,
    pub credentials: NotionBackupCredentials,
}

impl MockNotionBackup {
    pub fn insert_command(&self, command: MockNotionCommandEntry) -> Result<()> {
        let mut commands = self.storage.commands.write().map_err(|_err| {
            Error::Backup(eyre!(
                "Commands blocked for writing, can't proceed with command {{{}}} backup",
                command.external_id
            ))
        })?;

        commands.insert(command.external_id.clone(), command);

        Ok(())
    }

    pub fn insert_workspace(&self, workspace: MockNotionWorkspaceEntry) -> Result<()> {
        let mut workspaces = self.storage.workspaces.write().map_err(|_err| {
            Error::Backup(eyre!(
                "Workspaces blocked for writing, can't proceed with workspace {{{}}} backup",
                workspace.external_id
            ))
        })?;

        workspaces.insert(workspace.external_id.clone(), workspace);

        Ok(())
    }

    pub fn list_commands(&self, index: usize) -> Result<Vec<Command>> {
        let mut commands: Vec<MockNotionCommandEntry> = self
            .storage
            .commands
            .read()
            .map_err(|_err| {
                Error::Backup(eyre!(
                    "Commands blocked for reading, can't proceed with command listing"
                ))
            })?
            .values()
            .cloned()
            .collect();

        commands.sort_by(|a, b| a.program.cmp(&b.program));

        let commands = commands
            .into_iter()
            .skip(index * DEFAULT_PAGE_SIZE)
            .take(DEFAULT_PAGE_SIZE)
            .map(TryFrom::try_from)
            .collect::<Result<Vec<Command>>>()?;

        Ok(commands)
    }

    pub fn list_workspaces(&self, index: usize) -> Result<Vec<Workspace>> {
        let mut workspaces: Vec<MockNotionWorkspaceEntry> = self
            .storage
            .workspaces
            .read()
            .map_err(|_err| {
                Error::Backup(eyre!(
                    "Workspaces blocked for reading, can't proceed with workspaces listing"
                ))
            })?
            .values()
            .cloned()
            .collect();

        workspaces.sort_by(|a, b| a.name.cmp(&b.name));

        let workspaces = workspaces
            .into_iter()
            .skip(index * DEFAULT_PAGE_SIZE)
            .take(DEFAULT_PAGE_SIZE)
            .map(TryFrom::try_from)
            .collect::<Result<Vec<Workspace>>>()?;

        Ok(workspaces)
    }
}

impl MockNotionBackupBuilder {
    pub fn build(&self, credentials: BackupCredentials) -> MockNotionBackup {
        let BackupCredentials::Notion(credentials) = credentials;

        MockNotionBackup {
            storage: self.storage.clone(),
            credentials,
        }
    }
}

fn index_from_str(page_id: Option<&str>) -> Result<usize> {
    let page_id = page_id.unwrap_or("0");

    page_id
        .parse()
        .map_err(|_err| Error::Backup(eyre!("Invalid requested page ID: {}", page_id)))
}

impl BackupServiceBuilder<MockNotionBackup> for MockNotionBackupBuilder {
    fn build_backup_provider(&self, credentials: &BackupCredentials) -> Result<MockNotionBackup> {
        Ok(self.build(credentials.clone()))
    }
}

impl BackupService for MockNotionBackup {}

impl ListCommandsBackup for MockNotionBackup {
    fn list_commands_backup(
        &self,
        page_id: Option<&str>,
    ) -> Result<Option<(Vec<Command>, Option<String>)>> {
        let index: usize = index_from_str(page_id)?;
        let commands = self.list_commands(index)?;

        if commands.is_empty() {
            return Ok(None);
        }

        Ok(Some((commands, Some((index + 1).to_string()))))
    }
}

impl ListWorkspacesBackup for MockNotionBackup {
    fn list_workspaces_backup(
        &self,
        page_id: Option<&str>,
    ) -> Result<Option<(Vec<Workspace>, Option<String>)>> {
        let index = index_from_str(page_id)?;
        let workspaces: Vec<Workspace> = self.list_workspaces(index)?;

        if workspaces.is_empty() {
            return Ok(None);
        }

        Ok(Some((workspaces, Some((index + 1).to_string()))))
    }
}

impl UpsertCommandsBackup for MockNotionBackup {
    fn upsert_commands_backup(&self, commands: Vec<Command>) -> hermione_nexus::Result<()> {
        for command in commands {
            self.insert_command(command.into())?;
        }

        Ok(())
    }
}

impl UpsertWorkspacesBackup for MockNotionBackup {
    fn upsert_workspaces_backup(&self, workspaces: Vec<Workspace>) -> hermione_nexus::Result<()> {
        for workspace in workspaces {
            self.insert_workspace(workspace.into())?;
        }

        Ok(())
    }
}

impl VerifyBackupCredentials for MockNotionBackup {
    fn verify_backup_credentials(&self) -> Result<bool> {
        if self.credentials.api_key() != self.storage.api_key {
            return Err(Error::Backup(eyre!(
                "Failed to verify backup credentials: invalid API key"
            )));
        }

        if self.credentials.commands_database_id() != self.storage.commands_database_id {
            return Err(Error::Backup(eyre!(
                "Failed to verify backup credentials: invalid commands database ID"
            )));
        }

        if self.credentials.workspaces_database_id() != self.storage.workspaces_database_id {
            return Err(Error::Backup(eyre!(
                "Failed to verify backup credentials: invalid workspaces database ID"
            )));
        }

        Ok(true)
    }
}

impl From<Command> for MockNotionCommandEntry {
    fn from(value: Command) -> Self {
        MockNotionCommandEntry {
            external_id: value.id().to_string(),
            name: value.name().to_string(),
            program: value.program().to_string(),
            workspace_id: value.workspace_id().to_string(),
        }
    }
}

impl From<Workspace> for MockNotionWorkspaceEntry {
    fn from(value: Workspace) -> Self {
        MockNotionWorkspaceEntry {
            external_id: value.id().to_string(),
            name: value.name().to_string(),
            location: value
                .location()
                .map(ToString::to_string)
                .unwrap_or_default(),
        }
    }
}

impl TryFrom<MockNotionCommandEntry> for Command {
    type Error = Error;

    fn try_from(value: MockNotionCommandEntry) -> Result<Self> {
        Command::new(CommandParameters {
            id: value.external_id.parse().map_err(|_err| {
                Error::Backup(eyre!("Invalid command ID: {{{}}}", value.external_id))
            })?,
            last_execute_time: None,
            name: value.name,
            program: value.program,
            workspace_id: value
                .workspace_id
                .parse::<Uuid>()
                .map_err(|_err| {
                    Error::Backup(eyre!(
                        "Command {{{}}} has an invalid workspace ID: {{{}}}",
                        value.external_id,
                        value.workspace_id,
                    ))
                })?
                .into(),
        })
    }
}

impl TryFrom<MockNotionWorkspaceEntry> for Workspace {
    type Error = Error;

    fn try_from(value: MockNotionWorkspaceEntry) -> Result<Self> {
        Workspace::new(WorkspaceParameters {
            id: value.external_id.parse().map_err(|_err| {
                Error::Backup(eyre!("Invalid workspace ID: {{{}}}", value.external_id))
            })?,
            name: value.name,
            location: Some(value.location),
            last_access_time: None,
        })
    }
}
