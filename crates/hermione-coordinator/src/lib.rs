use hermione_core::{
    commands::{
        Command, CommandWorkspaceScopedId, CopyProgramToClipboardOperation, CreateCommandOperation,
        DeleteCommandFromWorkspaceOperation, ExecuteCommandWithinWorkspaceOperation,
        ExecuteCommandWithinWorkspaceParameters, FindCommandInWorkspaceOperation,
        GetCommandFromWorkspaceOperation, ImportCommandOperation, ListCommandsOperation,
        ListCommandsParameters, ListCommandsWithinWorkspaceOperation,
        ListCommandsWithinWorkspaceParameters, LoadCommandParameters, NewCommandParameters,
        UpdateCommandOperation,
    },
    extensions::{OpenWindowsTerminalOperation, OpenWindowsTerminalParameters},
    workspaces::{
        CreateWorkspaceOperation, DeleteWorkspaceOperation, FindWorkspaceOperation,
        GetWorkspaceOperation, ImportWorkspaceOperation, ListWorkspaceOperation,
        ListWorkspacesParameters, LoadWorkspaceParameters, NewWorkspaceParameters,
        UpdateWorkspaceOperation, Workspace,
    },
};
use hermione_powershell::PowerShellProvider;
use hermione_storage::StorageProvider;
use std::path::Path;

const DATABASE_FILE_NAME: &str = "hermione.db3";

pub struct Coordinator {
    storage: StorageProvider,
    powershell: PowerShellProvider,
}

pub struct CommandDto {
    pub id: String,
    pub name: String,
    pub program: String,
    pub workspace_id: String,
}

pub struct ExecuteCommandWithinWorkspaceInput<'a> {
    pub command_id: &'a str,
    pub workspace_id: &'a str,
    pub no_exit: bool,
}

pub struct ListCommandsInput {
    pub page_number: u32,
    pub page_size: u32,
}

pub struct ListWorkspacesInput<'a> {
    pub name_contains: &'a str,
    pub page_number: u32,
    pub page_size: u32,
}

pub struct ListCommandsWithinWorkspaceInput<'a> {
    pub page_number: u32,
    pub page_size: u32,
    pub program_contains: &'a str,
    pub workspace_id: &'a str,
}

pub struct OpenWindowsTerminalInput<'a> {
    pub working_directory: &'a str,
}

pub struct WorkspaceDto {
    pub id: String,
    pub location: Option<String>,
    pub name: String,
}

impl Coordinator {
    pub fn new(dir_path: &Path) -> anyhow::Result<Self> {
        let path = dir_path.join(DATABASE_FILE_NAME);
        let storage = StorageProvider::new(&path)?;
        let powershell = PowerShellProvider::new()?;

        Ok(Self {
            storage,
            powershell,
        })
    }

    pub fn copy_program_to_clipboard(
        &self,
        workspace_id: &str,
        command_id: &str,
    ) -> anyhow::Result<()> {
        CopyProgramToClipboardOperation {
            clipboard_provider: &self.powershell,
            getter: &self.storage,
        }
        .execute(CommandWorkspaceScopedId {
            workspace_id: workspace_id.parse()?,
            command_id: command_id.parse()?,
        })?;

        Ok(())
    }

    pub fn create_command(&self, data: CommandDto) -> anyhow::Result<CommandDto> {
        let command = CreateCommandOperation {
            creator: &self.storage,
        }
        .execute(data.into_new_entity()?)?;

        Ok(CommandDto::from_entity(command))
    }

    pub fn create_workspace(&self, data: WorkspaceDto) -> anyhow::Result<WorkspaceDto> {
        let workspace = CreateWorkspaceOperation {
            creator: &self.storage,
        }
        .execute(data.new_entity())?;

        Ok(WorkspaceDto::from_entity(workspace))
    }

    pub fn delete_command_from_workspace(
        &self,
        workspace_id: &str,
        id: &str,
    ) -> anyhow::Result<()> {
        let id = CommandWorkspaceScopedId {
            workspace_id: workspace_id.parse()?,
            command_id: id.parse()?,
        };

        DeleteCommandFromWorkspaceOperation {
            deleter: &self.storage,
        }
        .execute(id)?;

        Ok(())
    }

    pub fn delete_workspace(&self, id: &str) -> anyhow::Result<()> {
        DeleteWorkspaceOperation {
            deleter: &self.storage,
        }
        .execute(id.parse()?)?;

        Ok(())
    }

    pub fn execute_command(&self, input: ExecuteCommandWithinWorkspaceInput) -> anyhow::Result<()> {
        let ExecuteCommandWithinWorkspaceInput {
            command_id,
            workspace_id,
            no_exit,
        } = input;

        ExecuteCommandWithinWorkspaceOperation {
            get_command: &self.storage,
            runner: &self.powershell,
            command_tracker: &self.storage,
            get_workspace: &self.storage,
            workspace_tracker: &self.storage,
        }
        .execute(ExecuteCommandWithinWorkspaceParameters {
            command_id: command_id.parse()?,
            workspace_id: workspace_id.parse()?,
            no_exit,
        })?;

        Ok(())
    }

    pub fn find_command_in_workspace(
        &self,
        workspace_id: &str,
        id: &str,
    ) -> anyhow::Result<Option<CommandDto>> {
        let id = CommandWorkspaceScopedId {
            workspace_id: workspace_id.parse()?,
            command_id: id.parse()?,
        };

        let command = FindCommandInWorkspaceOperation {
            finder: &self.storage,
        }
        .execute(id)?;

        Ok(command.map(CommandDto::from_entity))
    }

    pub fn find_workspace(&self, id: &str) -> anyhow::Result<Option<WorkspaceDto>> {
        let workspace = FindWorkspaceOperation {
            finder: &self.storage,
        }
        .execute(id.parse()?)?;

        Ok(workspace.map(WorkspaceDto::from_entity))
    }

    pub fn get_command_from_workspace(
        &self,
        workspace_id: &str,
        id: &str,
    ) -> anyhow::Result<CommandDto> {
        let id = CommandWorkspaceScopedId {
            workspace_id: workspace_id.parse()?,
            command_id: id.parse()?,
        };

        let command = GetCommandFromWorkspaceOperation {
            getter: &self.storage,
        }
        .execute(id)?;

        Ok(CommandDto::from_entity(command))
    }

    pub fn get_workspace(&self, id: &str) -> anyhow::Result<WorkspaceDto> {
        let workspace = GetWorkspaceOperation {
            getter: &self.storage,
        }
        .execute(id.parse()?)?;

        Ok(WorkspaceDto::from_entity(workspace))
    }

    pub fn import_command(&self, data: CommandDto) -> anyhow::Result<CommandDto> {
        let command = ImportCommandOperation {
            importer: &self.storage,
        }
        .execute(data.load_entity()?)?;

        Ok(CommandDto::from_entity(command))
    }

    pub fn import_workspace(&self, data: WorkspaceDto) -> anyhow::Result<WorkspaceDto> {
        let workspace = ImportWorkspaceOperation {
            importer: &self.storage,
        }
        .execute(data.load_entity()?)?;

        Ok(WorkspaceDto::from_entity(workspace))
    }

    pub fn list_commands(&self, parameters: ListCommandsInput) -> anyhow::Result<Vec<CommandDto>> {
        let ListCommandsInput {
            page_number,
            page_size,
        } = parameters;

        let workspaces = ListCommandsOperation {
            lister: &self.storage,
        }
        .execute(ListCommandsParameters {
            page_number,
            page_size,
        })?;

        Ok(workspaces
            .into_iter()
            .map(CommandDto::from_entity)
            .collect())
    }

    pub fn list_workspaces(
        &self,
        parameters: ListWorkspacesInput<'_>,
    ) -> anyhow::Result<Vec<WorkspaceDto>> {
        let ListWorkspacesInput {
            name_contains,
            page_number,
            page_size,
        } = parameters;

        let workspaces = ListWorkspaceOperation {
            lister: &self.storage,
        }
        .execute(ListWorkspacesParameters {
            name_contains,
            page_number,
            page_size,
        })?;

        Ok(workspaces
            .into_iter()
            .map(WorkspaceDto::from_entity)
            .collect())
    }

    pub fn list_commands_within_workspace(
        &self,
        parameters: ListCommandsWithinWorkspaceInput,
    ) -> anyhow::Result<Vec<CommandDto>> {
        let ListCommandsWithinWorkspaceInput {
            page_number,
            page_size,
            program_contains,
            workspace_id,
        } = parameters;

        let workspaces = ListCommandsWithinWorkspaceOperation {
            lister: &self.storage,
        }
        .execute(ListCommandsWithinWorkspaceParameters {
            page_number,
            page_size,
            program_contains,
            workspace_id: workspace_id.parse()?,
        })?;

        Ok(workspaces
            .into_iter()
            .map(CommandDto::from_entity)
            .collect())
    }

    pub fn open_windows_terminal(
        &self,
        parameters: OpenWindowsTerminalInput,
    ) -> anyhow::Result<()> {
        let OpenWindowsTerminalInput { working_directory } = parameters;

        OpenWindowsTerminalOperation {
            windows_terminal_provider: &self.powershell,
        }
        .execute(OpenWindowsTerminalParameters { working_directory })?;

        Ok(())
    }

    pub fn update_command(&self, data: CommandDto) -> anyhow::Result<CommandDto> {
        let command = UpdateCommandOperation {
            updater: &self.storage,
        }
        .execute(data.load_entity()?)?;

        Ok(CommandDto::from_entity(command))
    }

    pub fn update_workspace(&self, data: WorkspaceDto) -> anyhow::Result<WorkspaceDto> {
        let workspace = UpdateWorkspaceOperation {
            updater: &self.storage,
        }
        .execute(data.load_entity()?)?;

        Ok(WorkspaceDto::from_entity(workspace))
    }
}

impl WorkspaceDto {
    fn from_entity(workspace: Workspace) -> Self {
        Self {
            id: workspace.id().map(|id| id.to_string()).unwrap_or_default(),
            location: workspace.location().map(ToString::to_string),
            name: workspace.name().to_string(),
        }
    }

    fn load_entity(self) -> anyhow::Result<Workspace> {
        let WorkspaceDto { id, location, name } = self;

        Ok(Workspace::load(LoadWorkspaceParameters {
            id: id.parse()?,
            name,
            location,
            last_access_time: None,
        }))
    }

    fn new_entity(self) -> Workspace {
        let WorkspaceDto {
            id: _,
            location,
            name,
        } = self;

        Workspace::new(NewWorkspaceParameters { name, location })
    }
}

impl CommandDto {
    fn from_entity(command: Command) -> Self {
        Self {
            id: command.id().map(|id| id.to_string()).unwrap_or_default(),
            name: command.name().to_string(),
            program: command.program().to_string(),
            workspace_id: command.workspace_id().to_string(),
        }
    }

    fn load_entity(self) -> anyhow::Result<Command> {
        let CommandDto {
            id,
            name,
            program,
            workspace_id,
        } = self;

        Ok(Command::load(LoadCommandParameters {
            id: id.parse()?,
            name,
            last_execute_time: None,
            program,
            workspace_id: workspace_id.parse()?,
        }))
    }

    fn into_new_entity(self) -> anyhow::Result<Command> {
        let CommandDto {
            id: _,
            name,
            program,
            workspace_id,
        } = self;

        Ok(Command::new(NewCommandParameters {
            name,
            program,
            workspace_id: workspace_id.parse()?,
        }))
    }
}
