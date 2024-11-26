use eyre::eyre;
use hermione_nexus::{
    definitions::{
        BackupCredentials, Command, CommandParameters, NotionBackupCredentials, Workspace,
        WorkspaceParameters,
    },
    services::{
        BackupCommand, BackupCommands, BackupService, BackupServiceBuilder, BackupWorkspace,
        BackupWorkspaces, ListCommandsBackup, ListWorkspacesBackup, VerifyBackupCredentials,
    },
    Error, Result,
};
use std::{collections::HashMap, rc::Rc, sync::RwLock};
use uuid::Uuid;

const DEFAULT_PAGE_SIZE: usize = 10;

pub struct MockNotionStorage {
    pub api_key: String,
    pub commands_database_id: String,
    pub workspaces_database_id: String,
    pub commands: RwLock<HashMap<String, NotionCommand>>,
    pub workspaces: RwLock<HashMap<String, NotionWorkspace>>,
}

#[derive(Clone)]
pub struct NotionCommand {
    pub external_id: String,
    pub name: String,
    pub program: String,
    pub workspace_id: String,
}

#[derive(Clone)]
pub struct NotionWorkspace {
    pub external_id: String,
    pub name: String,
    pub location: String,
}

impl MockNotionStorage {
    pub fn empty() -> Self {
        Self::default()
    }
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

pub struct MockNotionBuilder {
    pub storage: Rc<MockNotionStorage>,
}

pub struct MockNotion {
    pub storage: Rc<MockNotionStorage>,
    pub credentials: NotionBackupCredentials,
}

impl MockNotion {
    pub fn insert_command(&self, command: NotionCommand) -> Result<()> {
        self.verify_api_key()?;
        self.verify_commands_database_id()?;

        let mut commands = self.storage.commands.write().map_err(|_err| {
            Error::backup_service_communication(eyre!(
                "Commands blocked for writing, can't proceed with command {{{}}} backup",
                command.external_id
            ))
        })?;

        commands.insert(command.external_id.clone(), command);

        Ok(())
    }

    pub fn insert_workspace(&self, workspace: NotionWorkspace) -> Result<()> {
        self.verify_api_key()?;
        self.verify_workspaces_database_id()?;

        let mut workspaces = self.storage.workspaces.write().map_err(|_err| {
            Error::backup_service_communication(eyre!(
                "Workspaces blocked for writing, can't proceed with workspace {{{}}} backup",
                workspace.external_id
            ))
        })?;

        workspaces.insert(workspace.external_id.clone(), workspace);

        Ok(())
    }

    pub fn list_commands(&self, index: usize) -> Result<Vec<Command>> {
        self.verify_api_key()?;
        self.verify_commands_database_id()?;

        let mut commands: Vec<NotionCommand> = self
            .storage
            .commands
            .read()
            .map_err(|_err| {
                Error::backup_service_communication(eyre!(
                    "Commands blocked for reading, can't proceed with commands listing"
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
        self.verify_api_key()?;
        self.verify_workspaces_database_id()?;

        let mut workspaces: Vec<NotionWorkspace> = self
            .storage
            .workspaces
            .read()
            .map_err(|_err| {
                Error::backup_service_communication(eyre!(
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

    fn verify_api_key(&self) -> Result<()> {
        if self.credentials.api_key() == self.storage.api_key {
            return Ok(());
        }

        Err(Error::backup_service_configuration(eyre!(
            "Invalid API key"
        )))
    }

    fn verify_workspaces_database_id(&self) -> Result<()> {
        if self.credentials.workspaces_database_id() == self.storage.workspaces_database_id {
            return Ok(());
        }

        Err(Error::backup_service_configuration(eyre!(
            "Invalid workspaces database ID"
        )))
    }

    fn verify_commands_database_id(&self) -> Result<()> {
        if self.credentials.commands_database_id() == self.storage.commands_database_id {
            return Ok(());
        }

        Err(Error::backup_service_configuration(eyre!(
            "Invalid commands database ID"
        )))
    }
}

impl MockNotionBuilder {
    pub fn build(&self, credentials: BackupCredentials) -> MockNotion {
        let BackupCredentials::Notion(credentials) = credentials;

        MockNotion {
            storage: self.storage.clone(),
            credentials,
        }
    }
}

fn index_from_str(page_id: Option<&str>) -> Result<usize> {
    let page_id = page_id.unwrap_or("0");

    page_id.parse().map_err(|_err| {
        Error::backup_service_communication(eyre!("Invalid requested page ID: {}", page_id))
    })
}

impl BackupServiceBuilder<MockNotion> for MockNotionBuilder {
    fn build_backup_provider(&self, credentials: &BackupCredentials) -> Result<MockNotion> {
        Ok(self.build(credentials.clone()))
    }
}

impl BackupService for MockNotion {}

impl ListCommandsBackup for MockNotion {
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

impl ListWorkspacesBackup for MockNotion {
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

impl BackupCommands for MockNotion {
    fn backup_commands(&self, commands: Vec<Command>) -> hermione_nexus::Result<()> {
        for command in commands {
            self.insert_command(command.into())?;
        }

        Ok(())
    }
}

impl BackupCommand for MockNotion {
    fn backup_command(&self, command: Command) -> hermione_nexus::Result<()> {
        self.insert_command(command.into())
    }
}

impl BackupWorkspaces for MockNotion {
    fn backup_workspaces(&self, workspaces: Vec<Workspace>) -> hermione_nexus::Result<()> {
        for workspace in workspaces {
            self.insert_workspace(workspace.into())?;
        }

        Ok(())
    }
}

impl BackupWorkspace for MockNotion {
    fn backup_workspace(&self, workspace: Workspace) -> hermione_nexus::Result<()> {
        self.insert_workspace(workspace.into())
    }
}

impl VerifyBackupCredentials for MockNotion {
    fn verify_backup_credentials(&self) -> Result<()> {
        self.verify_api_key()?;
        self.verify_commands_database_id()?;
        self.verify_workspaces_database_id()?;

        Ok(())
    }
}

impl From<Command> for NotionCommand {
    fn from(value: Command) -> Self {
        NotionCommand {
            external_id: value.id().to_string(),
            name: value.name().to_string(),
            program: value.program().to_string(),
            workspace_id: value.workspace_id().to_string(),
        }
    }
}

impl From<Workspace> for NotionWorkspace {
    fn from(value: Workspace) -> Self {
        NotionWorkspace {
            external_id: value.id().to_string(),
            name: value.name().to_string(),
            location: value
                .location()
                .map(ToString::to_string)
                .unwrap_or_default(),
        }
    }
}

impl TryFrom<NotionCommand> for Command {
    type Error = Error;

    fn try_from(value: NotionCommand) -> Result<Self> {
        let id = value.external_id.parse().map_err(|_err| {
            Error::backup_service_data_corruption(eyre!(
                "Invalid command ID: {{{}}}",
                value.external_id
            ))
        })?;

        let workspace_id = value.workspace_id.parse::<Uuid>().map_err(|_err| {
            Error::backup_service_data_corruption(eyre!(
                "Command {{{}}} has an invalid workspace ID: {{{}}}",
                value.external_id,
                value.workspace_id,
            ))
        })?;

        Command::new(CommandParameters {
            id,
            last_execute_time: None,
            name: value.name,
            program: value.program,
            workspace_id: workspace_id.into(),
        })
    }
}

impl TryFrom<NotionWorkspace> for Workspace {
    type Error = Error;

    fn try_from(value: NotionWorkspace) -> Result<Self> {
        let id = value.external_id.parse().map_err(|_err| {
            Error::backup_service_data_corruption(eyre!(
                "Invalid workspace ID: {{{}}}",
                value.external_id
            ))
        })?;

        Workspace::new(WorkspaceParameters {
            id,
            name: value.name,
            location: Some(value.location),
            last_access_time: None,
        })
    }
}
