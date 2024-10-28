use crate::{
    providers::{clipboard::ClipboardProvider, system::SystemProvider},
    CommandPresenter, Result, WorkspacePresenter,
};
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
use hermione_storage::StorageProvider;

pub struct Coordinator<'a> {
    pub storage_provider: StorageProvider<'a>,
    pub clipboard_provider: ClipboardProvider<'a>,
    pub system_provider: SystemProvider<'a>,
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

impl<'a> Coordinator<'a> {
    pub fn copy_program_to_clipboard(&self, workspace_id: &str, command_id: &str) -> Result<()> {
        CopyCommandToClipboardOperation {
            clipboard_provider: &self.clipboard_provider,
            getter: &self.storage_provider,
        }
        .execute(CommandWorkspaceScopedId {
            workspace_id: workspace_id.parse()?,
            command_id: command_id.parse()?,
        })?;

        Ok(())
    }

    pub fn create_command(&self, dto: CommandPresenter) -> Result<CommandPresenter> {
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
            creator: &self.storage_provider,
        }
        .execute(new_command)?;

        Ok(command.into())
    }

    pub fn create_workspace(&self, dto: WorkspacePresenter) -> Result<WorkspacePresenter> {
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
            creator: &self.storage_provider,
        }
        .execute(new_workspace)?;

        Ok(workspace.into())
    }

    pub fn delete_command_from_workspace(&self, workspace_id: &str, id: &str) -> Result<()> {
        let id = CommandWorkspaceScopedId {
            workspace_id: workspace_id.parse()?,
            command_id: id.parse()?,
        };

        DeleteCommandFromWorkspaceOperation {
            deleter: &self.storage_provider,
        }
        .execute(id)?;

        Ok(())
    }

    pub fn delete_workspace(&self, id: &str) -> Result<()> {
        DeleteWorkspaceOperation {
            deleter: &self.storage_provider,
        }
        .execute(id.parse()?)?;

        Ok(())
    }

    pub fn execute_command(&self, input: ExecuteCommandWithinWorkspaceInput) -> Result<()> {
        let ExecuteCommandWithinWorkspaceInput {
            command_id,
            workspace_id,
            no_exit,
        } = input;

        ExecuteCommandOperation {
            get_command: &self.storage_provider,
            runner: &self.system_provider,
            command_tracker: &self.storage_provider,
            get_workspace: &self.storage_provider,
            workspace_tracker: &self.storage_provider,
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
    ) -> Result<CommandPresenter> {
        let id = CommandWorkspaceScopedId {
            workspace_id: workspace_id.parse()?,
            command_id: id.parse()?,
        };

        let command = GetCommandFromWorkspaceOperation {
            getter: &self.storage_provider,
        }
        .execute(id)?;

        Ok(command.into())
    }

    pub fn get_workspace(&self, id: &str) -> Result<WorkspacePresenter> {
        let workspace = GetWorkspaceOperation {
            getter: &self.storage_provider,
        }
        .execute(id.parse()?)?;

        Ok(workspace.into())
    }

    pub fn list_commands_within_workspace(
        &self,
        parameters: ListCommandsWithinWorkspaceInput,
    ) -> Result<Vec<CommandPresenter>> {
        let ListCommandsWithinWorkspaceInput {
            page_number,
            page_size,
            program_contains,
            workspace_id,
        } = parameters;

        let workspaces = ListCommandsWithinWorkspaceOperation {
            lister: &self.storage_provider,
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
    ) -> Result<Vec<WorkspacePresenter>> {
        let ListWorkspacesInput {
            name_contains,
            page_number,
            page_size,
        } = parameters;

        let workspaces = ListWorkspaceOperation {
            lister: &self.storage_provider,
        }
        .execute(ListWorkspacesParameters {
            name_contains,
            page_number,
            page_size,
        })?;

        Ok(workspaces.into_iter().map(Into::into).collect())
    }

    pub fn open_windows_terminal(&self, parameters: OpenWindowsTerminalInput) -> Result<()> {
        let OpenWindowsTerminalInput { working_directory } = parameters;

        OpenWindowsTerminalOperation {
            windows_terminal_provider: &self.system_provider,
        }
        .execute(OpenWindowsTerminalParameters { working_directory })?;

        Ok(())
    }

    pub fn update_command(&self, data: CommandPresenter) -> Result<CommandPresenter> {
        let command = UpdateCommandOperation {
            updater: &self.storage_provider,
        }
        .execute(data.try_into()?)?;

        Ok(command.into())
    }

    pub fn update_workspace(&self, dto: WorkspacePresenter) -> Result<WorkspacePresenter> {
        let workspace = UpdateWorkspaceOperation {
            updater: &self.storage_provider,
        }
        .execute(dto.try_into()?)?;

        Ok(workspace.into())
    }
}
