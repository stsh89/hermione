use eyre::{eyre, Report};
use hermione_nexus::{
    definitions::{
        BackupCredentials, Command, CommandParameters, NotionBackupCredentials, Workspace,
        WorkspaceId, WorkspaceParameters,
    },
    services::{
        BackupCommand, BackupCommands, BackupCopies, BackupCopyParameters, BackupService,
        BackupServiceBuilder, BackupWorkspace, BackupWorkspaces, GetCommandsBackupCopy,
        GetWorkspacesBackupCopy, VerifyBackupCredentials,
    },
    Error,
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
    pub fn insert_command(&self, command: NotionCommand) -> Result<(), Report> {
        self.verify_api_key()?;
        self.verify_commands_database_id()?;

        let mut commands = self
            .storage
            .commands
            .write()
            .map_err(|err| err.to_string())
            .map_err(Report::msg)
            .map_err(Error::storage)?;

        commands.insert(command.external_id.clone(), command);

        Ok(())
    }

    pub fn insert_workspace(&self, workspace: NotionWorkspace) -> Result<(), Report> {
        self.verify_api_key()?;
        self.verify_workspaces_database_id()?;

        let mut workspaces = self
            .storage
            .workspaces
            .write()
            .map_err(|err| err.to_string())
            .map_err(Report::msg)
            .map_err(Error::storage)?;

        workspaces.insert(workspace.external_id.clone(), workspace);

        Ok(())
    }

    pub fn list_commands(&self, index: usize) -> Result<Vec<Command>, Report> {
        self.verify_api_key()?;
        self.verify_commands_database_id()?;

        let mut commands: Vec<NotionCommand> = self
            .storage
            .commands
            .read()
            .map_err(|err| err.to_string())
            .map_err(Report::msg)
            .map_err(Error::storage)?
            .values()
            .cloned()
            .collect();

        commands.sort_by(|a, b| a.program.cmp(&b.program));

        let commands = commands
            .into_iter()
            .skip(index * DEFAULT_PAGE_SIZE)
            .take(DEFAULT_PAGE_SIZE)
            .map(TryFrom::try_from)
            .collect::<Result<Vec<Command>, _>>()?;

        Ok(commands)
    }

    pub fn list_workspaces(&self, index: usize) -> Result<Vec<Workspace>, Report> {
        self.verify_api_key()?;
        self.verify_workspaces_database_id()?;

        let mut workspaces: Vec<NotionWorkspace> = self
            .storage
            .workspaces
            .read()
            .map_err(|err| err.to_string())
            .map_err(Report::msg)
            .map_err(Error::storage)?
            .values()
            .cloned()
            .collect();

        workspaces.sort_by(|a, b| a.name.cmp(&b.name));

        let workspaces = workspaces
            .into_iter()
            .skip(index * DEFAULT_PAGE_SIZE)
            .take(DEFAULT_PAGE_SIZE)
            .map(TryFrom::try_from)
            .collect::<Result<Vec<Workspace>, _>>()?;

        Ok(workspaces)
    }

    fn verify_api_key(&self) -> Result<(), Report> {
        if self.credentials.api_key() == self.storage.api_key {
            return Ok(());
        }

        Err(Report::msg("Not authorized Notion API key"))
    }

    fn verify_workspaces_database_id(&self) -> Result<(), Report> {
        if self.credentials.workspaces_database_id() == self.storage.workspaces_database_id {
            return Ok(());
        }

        Err(eyre!(
            "Could not find Notion workspaces database with ID: {}",
            self.storage.workspaces_database_id
        ))
    }

    fn verify_commands_database_id(&self) -> Result<(), Report> {
        if self.credentials.commands_database_id() == self.storage.commands_database_id {
            return Ok(());
        }

        Err(eyre!(
            "Could not find Notion commands database with ID: {}",
            self.storage.commands_database_id
        ))
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

fn index_from_str(page_id: Option<&str>) -> Result<usize, Error> {
    let page_id = page_id.unwrap_or("0");

    page_id
        .parse()
        .map_err(|_err| Error::backup(eyre!("Invalid requested page ID: {}", page_id)))
}

impl BackupServiceBuilder<MockNotion> for MockNotionBuilder {
    fn build_backup_provider(&self, credentials: &BackupCredentials) -> Result<MockNotion, Error> {
        Ok(self.build(credentials.clone()))
    }
}

impl BackupService for MockNotion {}

impl GetCommandsBackupCopy for MockNotion {
    fn get_commands_backup_copy(
        &self,
        parameters: BackupCopyParameters,
    ) -> Result<BackupCopies<Command>, Error> {
        let BackupCopyParameters { page_token } = parameters;

        let index: usize = index_from_str(page_token)?;
        let commands = self.list_commands(index).map_err(Error::backup)?;

        if commands.is_empty() {
            return Ok(BackupCopies {
                copies: vec![],
                next_page_token: None,
            });
        }

        let next_page_token = (index + 1).to_string();

        Ok(BackupCopies {
            copies: commands,
            next_page_token: Some(next_page_token),
        })
    }
}

impl GetWorkspacesBackupCopy for MockNotion {
    fn get_workspaces_backup_copy(
        &self,
        parameters: BackupCopyParameters,
    ) -> Result<BackupCopies<Workspace>, Error> {
        let BackupCopyParameters { page_token } = parameters;

        let index = index_from_str(page_token)?;
        let workspaces: Vec<Workspace> = self.list_workspaces(index).map_err(Error::backup)?;

        if workspaces.is_empty() {
            return Ok(BackupCopies {
                copies: vec![],
                next_page_token: None,
            });
        }

        let next_page_token = (index + 1).to_string();

        Ok(BackupCopies {
            copies: workspaces,
            next_page_token: Some(next_page_token),
        })
    }
}

impl BackupCommands for MockNotion {
    fn backup_commands(&self, commands: Vec<Command>) -> hermione_nexus::Result<()> {
        for command in commands {
            self.insert_command(command.into()).map_err(Error::backup)?;
        }

        Ok(())
    }
}

impl BackupCommand for MockNotion {
    fn backup_command(&self, command: Command) -> hermione_nexus::Result<()> {
        self.insert_command(command.into()).map_err(Error::backup)
    }
}

impl BackupWorkspaces for MockNotion {
    fn backup_workspaces(&self, workspaces: Vec<Workspace>) -> hermione_nexus::Result<()> {
        for workspace in workspaces {
            self.insert_workspace(workspace.into())
                .map_err(Error::backup)?;
        }

        Ok(())
    }
}

impl BackupWorkspace for MockNotion {
    fn backup_workspace(&self, workspace: Workspace) -> Result<(), Error> {
        self.insert_workspace(workspace.into())
            .map_err(Error::backup)
    }
}

impl VerifyBackupCredentials for MockNotion {
    fn verify_backup_credentials(&self) -> Result<(), Error> {
        self.verify_api_key().map_err(Error::backup)?;
        self.verify_commands_database_id().map_err(Error::backup)?;
        self.verify_workspaces_database_id()
            .map_err(Error::backup)?;

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

    fn try_from(value: NotionCommand) -> Result<Self, Error> {
        let id = value.external_id.parse().map_err(|_err| {
            Error::backup(eyre!("Invalid Notion command ID: {}", value.external_id))
        })?;

        let workspace_id = value.workspace_id.parse::<Uuid>().map_err(|_err| {
            Error::backup(eyre!(
                "Invalid Notion command {} workspace ID: {}",
                id,
                value.workspace_id,
            ))
        })?;

        Command::new(CommandParameters {
            id,
            last_execute_time: None,
            name: value.name,
            program: value.program,
            workspace_id: WorkspaceId::new(workspace_id)?,
        })
    }
}

impl TryFrom<NotionWorkspace> for Workspace {
    type Error = Error;

    fn try_from(value: NotionWorkspace) -> Result<Self, Error> {
        let id = value.external_id.parse().map_err(|_err| {
            Error::backup(eyre!(
                "Invalid Notion workspace ID: {{{}}}",
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
