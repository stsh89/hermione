use chrono::Utc;
use eyre::eyre;
use hermione_nexus::{
    definitions::{
        BackupCredentials, BackupProviderKind, Command, CommandId, CommandParameters, Workspace,
        WorkspaceId, WorkspaceParameters,
    },
    services::{
        CreateCommand, CreateWorkspace, DeleteBackupCredentials, DeleteCommand, DeleteWorkspace,
        DeleteWorkspaceCommands, EditCommandParameters, EditWorkspaceParameters,
        FilterCommandsParameters, FilterWorkspacesParameters, FindBackupCredentials, FindCommand,
        FindWorkspace, ListBackupCredentials, ListCommands, ListWorkspaces, NewCommandParameters,
        NewWorkspaceParameters, SaveBackupCredentials, StorageService, TrackCommandExecuteTime,
        TrackWorkspaceAccessTime, UpdateCommand, UpdateWorkspace, UpsertCommands, UpsertWorkspaces,
    },
    Error, Result,
};
use std::{collections::HashMap, sync::RwLock};
use uuid::Uuid;

pub const NOTION_CREDENTIALS_KEY: &str = "notion";

#[derive(Default)]
pub struct InMemoryStorage {
    pub backup_credentials: RwLock<HashMap<String, BackupCredentials>>,
    pub commands: RwLock<HashMap<Uuid, Command>>,
    pub workspaces: RwLock<HashMap<Uuid, Workspace>>,
}

impl InMemoryStorage {
    pub fn list_backup_credentials(&self) -> Result<Vec<BackupCredentials>> {
        let credentials = self.backup_credentials.read()
            .map_err(|_err| Error::storage(eyre!("Backup credentials blocked for reading, can't proceed with backup credentials listing")))?;

        Ok(credentials.values().cloned().collect())
    }

    pub fn list_commands(&self) -> Result<Vec<Command>> {
        let commands = self.commands.read().map_err(|_err| {
            Error::storage(eyre!(
                "Commands blocked for reading, can't proceed with commands listing"
            ))
        })?;

        Ok(commands.values().cloned().collect())
    }

    pub fn empty() -> Self {
        Self::default()
    }

    fn get_backup_credentials(&self, kind: &str) -> Result<Option<BackupCredentials>> {
        let credentials = self
            .backup_credentials
            .read()
            .map_err(|_err| {
                Error::storage(eyre!(
                    "Backup credentials blocked for reading, can't get {} backup credentials",
                    kind
                ))
            })?
            .get(kind)
            .cloned();

        Ok(credentials)
    }

    pub fn get_command(&self, id: &Uuid) -> Result<Option<Command>> {
        let command = self
            .commands
            .read()
            .map_err(|_err| {
                Error::storage(eyre!(
                    "Commands blocked for reading, can't get command {}",
                    id.braced()
                ))
            })?
            .get(id)
            .cloned();

        Ok(command)
    }

    pub fn get_workspace(&self, id: &Uuid) -> Result<Option<Workspace>> {
        let workspace = self
            .workspaces
            .read()
            .map_err(|_err| {
                Error::storage(eyre!(
                    "Workspaces blocked for reading, can't get workspace {}",
                    id.braced()
                ))
            })?
            .get(id)
            .cloned();

        Ok(workspace)
    }

    pub fn insert_backup_credentials(&self, credentials: BackupCredentials) -> Result<()> {
        let mut collection = self.backup_credentials.write()
            .map_err(|_err| {
                Error::storage(eyre!(
                    "Backup credentials blocked for writing, can't proceed with backup credentials insert"
                ))
            })
        ?;

        let key = match &credentials {
            BackupCredentials::Notion(_) => NOTION_CREDENTIALS_KEY.to_string(),
        };

        collection.insert(key, credentials);

        Ok(())
    }

    pub fn insert_command(&self, command: &Command) -> Result<()> {
        let mut commands = self.commands.write().map_err(|_err| {
            Error::storage(eyre!(
                "Commands blocked for writing, can't proceed with command insert",
            ))
        })?;

        commands.insert(**command.id(), command.clone());

        Ok(())
    }

    pub fn insert_workspace(&self, workspace: &Workspace) -> Result<()> {
        let mut workspaces = self.workspaces.write().map_err(|_err| {
            Error::storage(eyre!(
                "Workspaces blocked for writing, can't proceed with workspace insert"
            ))
        })?;

        workspaces.insert(**workspace.id(), workspace.clone());

        Ok(())
    }

    fn remove_backup_credentials(&self, kind: &str) -> Result<()> {
        let mut credentials = self.backup_credentials.write().map_err(|_err| {
            Error::storage(eyre!(
                "Backup credentials blocked for writing, can't remove {} backup credentials",
                kind
            ))
        })?;

        credentials.remove(kind);

        Ok(())
    }

    fn remove_command(&self, id: &Uuid) -> Result<()> {
        let mut commands = self.commands.write().map_err(|_err| {
            Error::storage(eyre!(
                "Commands blocked for writing, can't remove command {}",
                id.braced()
            ))
        })?;

        commands.remove(id);

        Ok(())
    }

    fn remove_workspace_commands(&self, workspace_id: &WorkspaceId) -> Result<()> {
        let mut commands = self.commands.write().map_err(|_err| {
            Error::storage(eyre!(
                "Commands blocked for writing, can't remove commands from workspace {}",
                **workspace_id
            ))
        })?;

        commands.retain(|_id, command| command.workspace_id() != workspace_id);

        Ok(())
    }

    fn remove_workspace(&self, id: &Uuid) -> Result<()> {
        let mut workspace = self.workspaces.write().map_err(|_err| {
            Error::storage(eyre!(
                "Workspaces blocked for writing, can't remove workspace {}",
                id.braced()
            ))
        })?;

        workspace.remove(id);

        Ok(())
    }

    fn set_command_execute_time(&self, id: &Uuid) -> Result<()> {
        let command = self.get_command(id)?;

        let Some(mut command) = command else {
            return Ok(());
        };

        command.set_execute_time(Utc::now());

        self.insert_command(&command)?;

        Ok(())
    }

    fn set_workspace_access_time(&self, id: &Uuid) -> Result<()> {
        let workspace = self.get_workspace(id)?;

        let Some(mut workspace) = workspace else {
            return Ok(());
        };

        workspace.set_access_time(Utc::now());

        self.insert_workspace(&workspace)?;

        Ok(())
    }

    pub fn list_workspaces(&self) -> Result<Vec<Workspace>> {
        let workspaces = self.workspaces.read().map_err(|_err| {
            Error::storage(eyre!(
                "Workspaces blocked for reading, can't proceed with workspaces listing"
            ))
        })?;

        Ok(workspaces.values().cloned().collect())
    }
}

impl StorageService for InMemoryStorage {}

impl CreateCommand for InMemoryStorage {
    fn create_command(&self, parameters: NewCommandParameters) -> Result<Command> {
        let NewCommandParameters {
            name,
            program,
            workspace_id,
        } = parameters;

        let command = Command::new(CommandParameters {
            id: Uuid::new_v4(),
            last_execute_time: None,
            name,
            program,
            workspace_id,
        })?;

        self.insert_command(&command)?;

        Ok(command)
    }
}

impl CreateWorkspace for InMemoryStorage {
    fn create_workspace(&self, parameters: NewWorkspaceParameters) -> Result<Workspace> {
        let NewWorkspaceParameters { name, location } = parameters;

        let workspace = Workspace::new(WorkspaceParameters {
            id: Uuid::new_v4(),
            last_access_time: None,
            location,
            name,
        })?;

        self.insert_workspace(&workspace)?;

        Ok(workspace)
    }
}

impl DeleteBackupCredentials for InMemoryStorage {
    fn delete_backup_credentials(&self, kind: BackupProviderKind) -> Result<()> {
        let key = match kind {
            BackupProviderKind::Notion => NOTION_CREDENTIALS_KEY,
            BackupProviderKind::Unknown => return Ok(()),
        };

        self.remove_backup_credentials(key)?;

        Ok(())
    }
}

impl DeleteCommand for InMemoryStorage {
    fn delete_command(&self, id: &CommandId) -> Result<()> {
        self.remove_command(id)?;

        Ok(())
    }
}

impl DeleteWorkspaceCommands for InMemoryStorage {
    fn delete_workspace_commands(&self, id: &WorkspaceId) -> Result<()> {
        self.remove_workspace_commands(id)?;

        Ok(())
    }
}

impl DeleteWorkspace for InMemoryStorage {
    fn delete_workspace(&self, id: &WorkspaceId) -> Result<()> {
        self.remove_workspace(id)?;

        Ok(())
    }
}

impl FindBackupCredentials for InMemoryStorage {
    fn find_backup_credentials(
        &self,
        kind: BackupProviderKind,
    ) -> Result<Option<BackupCredentials>> {
        let key = match kind {
            BackupProviderKind::Notion => NOTION_CREDENTIALS_KEY,
            BackupProviderKind::Unknown => return Ok(None),
        };

        let credentials = self.get_backup_credentials(key)?;

        Ok(credentials)
    }
}

impl FindCommand for InMemoryStorage {
    fn find_command(&self, id: &CommandId) -> Result<Option<Command>> {
        let command = self.get_command(id)?;

        Ok(command)
    }
}

impl FindWorkspace for InMemoryStorage {
    fn find_workspace(&self, id: &WorkspaceId) -> Result<Option<Workspace>> {
        let workspaces = self.get_workspace(id)?;

        Ok(workspaces)
    }
}

impl ListBackupCredentials for InMemoryStorage {
    fn list_backup_credentials(&self) -> Result<Vec<BackupCredentials>> {
        let credentials = self.list_backup_credentials()?;

        Ok(credentials)
    }
}

impl ListCommands for InMemoryStorage {
    fn list_commands(&self, parameters: FilterCommandsParameters) -> Result<Vec<Command>> {
        let FilterCommandsParameters {
            program_contains,
            page_number,
            page_size,
            workspace_id,
        } = parameters;

        let mut commands = self
            .list_commands()?
            .into_iter()
            .filter(|command| {
                let contains_program = if let Some(program_contains) = program_contains {
                    command.program().contains(program_contains)
                } else {
                    true
                };

                let from_workspace = if let Some(workspace_id) = workspace_id {
                    command.workspace_id() == workspace_id
                } else {
                    true
                };

                contains_program && from_workspace
            })
            .collect::<Vec<Command>>();

        commands.sort_by(|a, b| a.program().cmp(b.program()));
        commands.sort_by(|a, b| a.last_execute_time().cmp(&b.last_execute_time()).reverse());

        Ok(commands
            .into_iter()
            .skip(page_number as usize * page_size as usize)
            .take(page_size as usize)
            .collect())
    }
}

impl ListWorkspaces for InMemoryStorage {
    fn list_workspaces(&self, parameters: FilterWorkspacesParameters) -> Result<Vec<Workspace>> {
        let FilterWorkspacesParameters {
            name_contains,
            page_number,
            page_size,
        } = parameters;

        let mut workspaces = self
            .list_workspaces()?
            .into_iter()
            .filter(|workspace| {
                if let Some(name_contains) = name_contains {
                    workspace.name().contains(name_contains)
                } else {
                    true
                }
            })
            .collect::<Vec<Workspace>>();

        workspaces.sort_by(|a, b| a.name().cmp(b.name()));
        workspaces.sort_by(|a, b| a.last_access_time().cmp(&b.last_access_time()).reverse());

        Ok(workspaces
            .into_iter()
            .skip(page_number as usize * page_size as usize)
            .take(page_size as usize)
            .collect())
    }
}

impl SaveBackupCredentials for InMemoryStorage {
    fn save_backup_credentials(&self, credentials: &BackupCredentials) -> Result<()> {
        self.insert_backup_credentials(credentials.clone())?;

        Ok(())
    }
}

impl TrackCommandExecuteTime for InMemoryStorage {
    fn track_command_execute_time(&self, id: &CommandId) -> Result<()> {
        self.set_command_execute_time(id)?;

        Ok(())
    }
}

impl TrackWorkspaceAccessTime for InMemoryStorage {
    fn track_workspace_access_time(&self, id: &WorkspaceId) -> Result<()> {
        self.set_workspace_access_time(id)?;

        Ok(())
    }
}

impl UpdateCommand for InMemoryStorage {
    fn update_command(&self, parameters: EditCommandParameters) -> Result<()> {
        let EditCommandParameters { id, name, program } = parameters;

        let Some(mut command) = self.get_command(id)? else {
            return Ok(());
        };

        command.set_name(name.to_string());
        command.set_program(program.to_string());

        self.insert_command(&command)?;

        Ok(())
    }
}

impl UpsertCommands for InMemoryStorage {
    fn upsert_commands(&self, commands: Vec<Command>) -> Result<()> {
        for command in commands {
            self.insert_command(&command)?;
        }

        Ok(())
    }
}

impl UpdateWorkspace for InMemoryStorage {
    fn update_workspace(&self, parameters: EditWorkspaceParameters) -> Result<()> {
        let EditWorkspaceParameters { id, location, name } = parameters;

        let Some(mut workspace) = self.get_workspace(id)? else {
            return Ok(());
        };

        workspace.set_location(location.map(ToString::to_string));
        workspace.set_name(name.to_string());

        self.insert_workspace(&workspace)?;

        Ok(())
    }
}

impl UpsertWorkspaces for InMemoryStorage {
    fn upsert_workspaces(&self, workspaces: Vec<Workspace>) -> Result<()> {
        for workspace in workspaces {
            self.insert_workspace(&workspace)?;
        }

        Ok(())
    }
}
