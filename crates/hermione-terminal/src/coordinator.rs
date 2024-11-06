use crate::{
    providers::{PowerShellClient, PowerShellParameters},
    services::{Clipboard, Storage, System},
    CommandPresenter, Result, WorkspacePresenter,
};
use hermione_nexus::operations::{
    CopyCommandToClipboardOperation, CreateCommandOperation, CreateCommandParameters,
    CreateWorkspaceOperation, CreateWorkspaceParameters, DeleteCommandOperation,
    DeleteWorkspaceOperation, ExecuteCommandOperation, GetCommandOperation, GetWorkspaceOperation,
    ListCommandsOperation, ListCommandsParameters, ListWorkspacesOperation,
    ListWorkspacesParameters, UpdateCommandOperation, UpdateCommandParameters,
    UpdateWorkspaceOperation, UpdateWorkspaceParameters,
};
use std::{
    num::{NonZero, NonZeroU32},
    str::FromStr,
};
use uuid::Uuid;

pub struct Coordinator {
    pub storage: Storage,
    pub powershell: PowerShellClient,
}

pub struct ExecuteCommandWithinWorkspaceInput<'a> {
    pub id: &'a str,
    pub workspace_id: &'a str,
    pub no_exit: bool,
}

pub struct ListWorkspacesInput<'a> {
    pub name_contains: &'a str,
    pub page_number: Option<NonZeroU32>,
    pub page_size: Option<NonZeroU32>,
}

pub struct ListCommandsWithinWorkspaceInput<'a> {
    pub page_number: Option<NonZeroU32>,
    pub page_size: Option<NonZeroU32>,
    pub program_contains: &'a str,
    pub workspace_id: &'a str,
}

pub struct OpenWindowsTerminalInput<'a> {
    pub working_directory: &'a str,
}

impl Coordinator {
    fn clipboard(&self) -> Clipboard {
        Clipboard {
            client: &self.powershell,
        }
    }

    pub fn copy_command_to_clipboard(&self, id: &str) -> Result<()> {
        CopyCommandToClipboardOperation {
            clipboard_provider: &self.clipboard(),
            storage_provider: &self.storage,
        }
        .execute(&Uuid::from_str(id)?.into())?;

        Ok(())
    }

    pub fn create_command(&self, dto: CommandPresenter) -> Result<CommandPresenter> {
        let CommandPresenter {
            id: _,
            name,
            program,
            workspace_id,
        } = dto;

        let command = CreateCommandOperation {
            provider: &self.storage,
        }
        .execute(CreateCommandParameters {
            name,
            program,
            workspace_id: Uuid::from_str(&workspace_id)?.into(),
        })?;

        Ok(command.into())
    }

    pub fn create_workspace(&self, dto: WorkspacePresenter) -> Result<WorkspacePresenter> {
        let WorkspacePresenter {
            id: _,
            location,
            name,
        } = dto;

        let workspace = CreateWorkspaceOperation {
            provider: &self.storage,
        }
        .execute(CreateWorkspaceParameters {
            name,
            location: Some(location),
        })?;

        Ok(workspace.into())
    }

    pub fn delete_command(&self, id: &str) -> Result<()> {
        DeleteCommandOperation {
            find_command_provider: &self.storage,
            delete_command_provider: &self.storage,
        }
        .execute(&Uuid::from_str(id)?.into())?;

        Ok(())
    }

    pub fn delete_workspace(&self, id: &str) -> Result<()> {
        DeleteWorkspaceOperation {
            find_workspace_provider: &self.storage,
            delete_workspace_provider: &self.storage,
            delete_workspace_commands_provider: &self.storage,
        }
        .execute(&Uuid::from_str(id)?.into())?;

        Ok(())
    }

    pub fn execute_command(&self, input: ExecuteCommandWithinWorkspaceInput) -> Result<()> {
        let ExecuteCommandWithinWorkspaceInput {
            id,
            workspace_id,
            no_exit,
        } = input;

        let workspace = self.get_workspace(workspace_id)?;

        let working_directory = if workspace.location.is_empty() {
            None
        } else {
            Some(workspace.location.as_str())
        };

        ExecuteCommandOperation {
            find_command_provider: &self.storage,
            system_provider: &System {
                client: &self.powershell,
                no_exit,
                working_directory,
            },
            track_command_provider: &self.storage,
            track_workspace_provider: &self.storage,
        }
        .execute(&Uuid::from_str(id)?.into())?;

        Ok(())
    }

    pub fn get_command(&self, id: &str) -> Result<CommandPresenter> {
        let command = GetCommandOperation {
            provider: &self.storage,
        }
        .execute(&Uuid::from_str(id)?.into())?;

        Ok(command.into())
    }

    pub fn get_workspace(&self, id: &str) -> Result<WorkspacePresenter> {
        let workspace = GetWorkspaceOperation {
            provider: &self.storage,
        }
        .execute(&Uuid::from_str(id)?.into())?;

        Ok(workspace.into())
    }

    pub fn list_workspace_commands(
        &self,
        parameters: ListCommandsWithinWorkspaceInput,
    ) -> Result<Vec<CommandPresenter>> {
        let ListCommandsWithinWorkspaceInput {
            page_number,
            page_size,
            program_contains,
            workspace_id,
        } = parameters;

        let workspaces = ListCommandsOperation {
            provider: &self.storage,
        }
        .execute(ListCommandsParameters {
            page_size: page_size.unwrap_or_else(|| NonZero::new(10).unwrap()),
            page_number: page_number.unwrap_or_else(|| NonZero::new(1).unwrap()),
            program_contains: Some(program_contains),
            workspace_id: Some(&Uuid::parse_str(workspace_id)?.into()),
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

        let workspaces = ListWorkspacesOperation {
            provider: &self.storage,
        }
        .execute(ListWorkspacesParameters {
            name_contains: Some(name_contains),
            page_number: page_number.unwrap_or_else(|| NonZero::new(1).unwrap()),
            page_size: page_size.unwrap_or_else(|| NonZero::new(10).unwrap()),
        })?;

        Ok(workspaces.into_iter().map(Into::into).collect())
    }

    pub fn open_windows_terminal(&self, parameters: OpenWindowsTerminalInput) -> Result<()> {
        let OpenWindowsTerminalInput { working_directory } = parameters;

        let working_directory = if working_directory.is_empty() {
            None
        } else {
            Some(working_directory)
        };

        self.powershell
            .open_windows_terminal(Some(PowerShellParameters {
                command: None,
                no_exit: false,
                working_directory,
            }))?;

        Ok(())
    }

    pub fn update_command(&self, data: CommandPresenter) -> Result<CommandPresenter> {
        let CommandPresenter {
            workspace_id: _,
            id,
            name,
            program,
        } = data;

        let command = UpdateCommandOperation {
            find_command_provider: &self.storage,
            update_command_provider: &self.storage,
        }
        .execute(UpdateCommandParameters {
            id: &Uuid::from_str(&id)?.into(),
            name,
            program,
        })?;

        Ok(command.into())
    }

    pub fn update_workspace(&self, presenter: WorkspacePresenter) -> Result<WorkspacePresenter> {
        let WorkspacePresenter { id, location, name } = presenter;

        let workspace = UpdateWorkspaceOperation {
            find_workspace_provider: &self.storage,
            update_workspace_provider: &self.storage,
        }
        .execute(UpdateWorkspaceParameters {
            id: &Uuid::from_str(&id)?.into(),
            location: Some(location),
            name,
        })?;

        Ok(workspace.into())
    }
}
