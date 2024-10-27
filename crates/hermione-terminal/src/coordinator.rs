use hermione_ops::{
    commands::{
        Command, CommandWorkspaceScopedId, CreateCommandOperation,
        DeleteCommandFromWorkspaceOperation, GetCommandFromWorkspaceOperation,
        ListCommandsWithinWorkspaceOperation, ListCommandsWithinWorkspaceParameters,
        NewCommandParameters, UpdateCommandOperation,
    },
    extensions::{
        CopyCommandToClipboardOperation, ExecuteCommandOperation,
        ExecuteCommandWithinWorkspaceParameters, OpenWindowsTerminalOperation,
        OpenWindowsTerminalParameters,
    },
    workspaces::{
        CreateWorkspaceOperation, DeleteWorkspaceOperation, GetWorkspaceOperation,
        ListWorkspaceOperation, ListWorkspacesParameters, NewWorkspaceParameters,
        UpdateWorkspaceOperation, Workspace,
    },
};
use hermione_powershell::PowerShellProvider;
use hermione_storage::sqlite::SqliteProvider;
use std::path::Path;

use crate::{CommandPresenter, WorkspacePresenter};

pub struct Coordinator {
    storage: SqliteProvider,
    powershell: PowerShellProvider,
}

pub struct ExecuteCommandWithinWorkspaceInput<'a> {
    pub command_id: &'a str,
    pub workspace_id: &'a str,
    pub no_exit: bool,
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

impl Coordinator {
    pub fn copy_program_to_clipboard(
        &self,
        workspace_id: &str,
        command_id: &str,
    ) -> anyhow::Result<()> {
        CopyCommandToClipboardOperation {
            clipboard_provider: &self.powershell,
            getter: &self.storage,
        }
        .execute(CommandWorkspaceScopedId {
            workspace_id: workspace_id.parse()?,
            command_id: command_id.parse()?,
        })?;

        Ok(())
    }

    pub fn create_command(&self, dto: CommandPresenter) -> anyhow::Result<CommandPresenter> {
        let CommandPresenter {
            id: _,
            name,
            program,
            workspace_id,
        } = dto;

        let new_command = Command::new(NewCommandParameters {
            name,
            program,
            workspace_id: workspace_id.parse()?,
        });

        let command = CreateCommandOperation {
            creator: &self.storage,
        }
        .execute(new_command)?;

        Ok(command.into())
    }

    pub fn create_workspace(&self, dto: WorkspacePresenter) -> anyhow::Result<WorkspacePresenter> {
        let WorkspacePresenter {
            id: _,
            location,
            name,
        } = dto;

        let new_workspace = Workspace::new(NewWorkspaceParameters {
            name,
            location: Some(location),
        });

        let workspace = CreateWorkspaceOperation {
            creator: &self.storage,
        }
        .execute(new_workspace)?;

        Ok(workspace.into())
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

        ExecuteCommandOperation {
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

    pub fn get_command_from_workspace(
        &self,
        workspace_id: &str,
        id: &str,
    ) -> anyhow::Result<CommandPresenter> {
        let id = CommandWorkspaceScopedId {
            workspace_id: workspace_id.parse()?,
            command_id: id.parse()?,
        };

        let command = GetCommandFromWorkspaceOperation {
            getter: &self.storage,
        }
        .execute(id)?;

        Ok(command.into())
    }

    pub fn get_workspace(&self, id: &str) -> anyhow::Result<WorkspacePresenter> {
        let workspace = GetWorkspaceOperation {
            getter: &self.storage,
        }
        .execute(id.parse()?)?;

        Ok(workspace.into())
    }

    pub fn list_commands_within_workspace(
        &self,
        parameters: ListCommandsWithinWorkspaceInput,
    ) -> anyhow::Result<Vec<CommandPresenter>> {
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

        Ok(workspaces.into_iter().map(Into::into).collect())
    }

    pub fn list_workspaces(
        &self,
        parameters: ListWorkspacesInput<'_>,
    ) -> anyhow::Result<Vec<WorkspacePresenter>> {
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

        Ok(workspaces.into_iter().map(Into::into).collect())
    }

    pub fn new(file_path: &Path) -> anyhow::Result<Self> {
        let storage = SqliteProvider::new(file_path)?;
        let powershell = PowerShellProvider::new()?;

        Ok(Self {
            storage,
            powershell,
        })
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

    pub fn update_command(&self, data: CommandPresenter) -> anyhow::Result<CommandPresenter> {
        let command = UpdateCommandOperation {
            updater: &self.storage,
        }
        .execute(data.try_into()?)?;

        Ok(command.into())
    }

    pub fn update_workspace(&self, dto: WorkspacePresenter) -> anyhow::Result<WorkspacePresenter> {
        let workspace = UpdateWorkspaceOperation {
            updater: &self.storage,
        }
        .execute(dto.try_into()?)?;

        Ok(workspace.into())
    }
}
