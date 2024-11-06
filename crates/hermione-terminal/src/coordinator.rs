use crate::{
    providers::powershell::{self, PowerShellParameters, PowerShellProcess},
    services::{Clipboard, Storage, System},
    BackupCredentialsKind, CommandPresenter, Result, WorkspacePresenter,
};
use hermione_nexus::operations::{
    CopyCommandToClipboardOperation, CreateCommandOperation, CreateCommandParameters,
    CreateWorkspaceOperation, CreateWorkspaceParameters, DeleteCommandOperation,
    DeleteWorkspaceOperation, ExecuteCommandOperation, GetCommandOperation, GetWorkspaceOperation,
    ListBackupCredentialsOperation, ListCommandsOperation, ListCommandsParameters,
    ListWorkspacesOperation, ListWorkspacesParameters, UpdateCommandOperation,
    UpdateCommandParameters, UpdateWorkspaceOperation, UpdateWorkspaceParameters,
};
use rusqlite::Connection;
use std::num::{NonZero, NonZeroU32};
use uuid::Uuid;

pub struct Coordinator {
    pub database_connection: Connection,
    pub powershell_process: PowerShellProcess,
}

pub struct ExecuteCommandInput {
    pub command_id: Uuid,
    pub workspace_id: Uuid,
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
    pub workspace_id: Uuid,
}

pub struct OpenWindowsTerminalInput<'a> {
    pub working_directory: &'a str,
}

impl Coordinator {
    fn clipboard(&self) -> Clipboard {
        Clipboard {
            process: &self.powershell_process,
        }
    }

    pub fn copy_command_to_clipboard(&self, id: Uuid) -> Result<()> {
        CopyCommandToClipboardOperation {
            clipboard_provider: &self.clipboard(),
            storage_provider: &self.storage(),
        }
        .execute(&id.into())?;

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
            storage_provider: &self.storage(),
        }
        .execute(CreateCommandParameters {
            name,
            program,
            workspace_id: workspace_id.into(),
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
            storage_provider: &self.storage(),
        }
        .execute(CreateWorkspaceParameters {
            name,
            location: Some(location),
        })?;

        Ok(workspace.into())
    }

    pub fn delete_command(&self, id: Uuid) -> Result<()> {
        let storage = self.storage();

        DeleteCommandOperation {
            find_provider: &storage,
            delete_provider: &storage,
        }
        .execute(&id.into())?;

        Ok(())
    }

    pub fn delete_workspace(&self, id: Uuid) -> Result<()> {
        let storage = self.storage();

        DeleteWorkspaceOperation {
            find_workspace_provider: &storage,
            delete_workspace_provider: &storage,
            delete_workspace_commands_provider: &storage,
        }
        .execute(&id.into())?;

        Ok(())
    }

    pub fn execute_command(&self, input: ExecuteCommandInput) -> Result<()> {
        let ExecuteCommandInput {
            command_id,
            workspace_id,
            no_exit,
        } = input;

        let workspace = self.get_workspace(workspace_id)?;

        let working_directory = if workspace.location.is_empty() {
            None
        } else {
            Some(workspace.location.as_str())
        };

        let storage = self.storage();

        ExecuteCommandOperation {
            find_command_provider: &storage,
            system_provider: &System {
                process: &self.powershell_process,
                no_exit,
                working_directory,
            },
            track_command_provider: &storage,
            track_workspace_provider: &storage,
        }
        .execute(&command_id.into())?;

        Ok(())
    }

    pub fn get_command(&self, id: Uuid) -> Result<CommandPresenter> {
        let command = GetCommandOperation {
            provider: &self.storage(),
        }
        .execute(&id.into())?;

        Ok(command.into())
    }

    pub fn get_workspace(&self, id: Uuid) -> Result<WorkspacePresenter> {
        let workspace = GetWorkspaceOperation {
            provider: &self.storage(),
        }
        .execute(&id.into())?;

        Ok(workspace.into())
    }

    pub fn list_backup_credentials(&self) -> Result<Vec<BackupCredentialsKind>> {
        let backup_credentials = ListBackupCredentialsOperation {
            provider: &self.storage(),
        }
        .execute()?;

        Ok(backup_credentials.into_iter().map(Into::into).collect())
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
            provider: &self.storage(),
        }
        .execute(ListCommandsParameters {
            page_size: page_size.unwrap_or_else(|| NonZero::new(10).unwrap()),
            page_number: page_number.unwrap_or_else(|| NonZero::new(1).unwrap()),
            program_contains: Some(program_contains),
            workspace_id: Some(&workspace_id.into()),
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
            provider: &self.storage(),
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

        powershell::open_windows_terminal(
            &self.powershell_process,
            Some(PowerShellParameters {
                command: None,
                no_exit: false,
                working_directory,
            }),
        )?;

        Ok(())
    }

    fn storage(&self) -> Storage {
        Storage {
            conn: &self.database_connection,
        }
    }

    pub fn update_command(&self, data: CommandPresenter) -> Result<CommandPresenter> {
        let CommandPresenter {
            workspace_id: _,
            id,
            name,
            program,
        } = data;

        let storage = self.storage();

        let command = UpdateCommandOperation {
            find_command_provider: &storage,
            update_command_provider: &storage,
        }
        .execute(UpdateCommandParameters {
            id: &id.into(),
            name,
            program,
        })?;

        Ok(command.into())
    }

    pub fn update_workspace(&self, presenter: WorkspacePresenter) -> Result<WorkspacePresenter> {
        let WorkspacePresenter { id, location, name } = presenter;

        let storage = self.storage();

        let workspace = UpdateWorkspaceOperation {
            find_workspace_provider: &storage,
            update_workspace_provider: &storage,
        }
        .execute(UpdateWorkspaceParameters {
            id: &id.into(),
            location: Some(location),
            name,
        })?;

        Ok(workspace.into())
    }
}
