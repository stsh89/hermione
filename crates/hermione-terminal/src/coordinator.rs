use crate::{
    models::{BackupCredentials, BackupProviderKind, Command, NotionBackupCredentials, Workspace},
    providers::powershell::{self, PowerShellParameters, PowerShellProcess},
    services::{Clipboard, NotionBackupBuilder, Storage, System},
    Result,
};
use hermione_nexus::operations::{
    CopyCommandToClipboardOperation, CreateCommandOperation, CreateCommandParameters,
    CreateWorkspaceOperation, CreateWorkspaceParameters, DeleteBackupCredentialsOperation,
    DeleteCommandOperation, DeleteWorkspaceOperation, ExecuteCommandOperation, ExportOperation,
    GetBackupCredentialsOperation, GetCommandOperation, GetWorkspaceOperation, ImportOperation,
    ListBackupCredentialsOperation, ListCommandsOperation, ListCommandsParameters,
    ListWorkspacesOperation, ListWorkspacesParameters, SaveBackupCredentialsOperation,
    UpdateCommandOperation, UpdateCommandParameters, UpdateWorkspaceOperation,
    UpdateWorkspaceParameters,
};
use rusqlite::Connection;
use std::num::NonZeroU32;
use uuid::Uuid;

pub const FIRST_PAGE: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(1) };
pub const DEFAULT_PAGE_SIZE: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(43) };

const DEFAULT_BACKUP_PAGE_SIZE: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100) };

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

    pub fn create_command(&self, command: Command) -> Result<Command> {
        let Command {
            id: _,
            name,
            program,
            workspace_id,
        } = command;

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

    pub fn create_workspace(&self, dto: Workspace) -> Result<Workspace> {
        let Workspace {
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

    pub fn delete_backup_credentials(&self, kind: BackupProviderKind) -> Result<()> {
        let storage = self.storage();

        DeleteBackupCredentialsOperation {
            find_provider: &storage,
            delete_provider: &storage,
        }
        .execute(&kind.into())?;

        Ok(())
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

    pub fn export(&self, kind: BackupProviderKind) -> Result<()> {
        let storage = self.storage();

        ExportOperation {
            backup_credentials_provider: &storage,
            list_commands_provider: &storage,
            list_workspaces_provider: &storage,
            backup_provider_builder: &NotionBackupBuilder {
                page_size: DEFAULT_BACKUP_PAGE_SIZE,
            },
            backup_provider: std::marker::PhantomData,
        }
        .execute(&kind.into())?;

        Ok(())
    }

    pub fn find_notion_backup_credentials(&self) -> Result<Option<NotionBackupCredentials>> {
        let kinds = self.list_backup_credentials()?;

        let notion_credential_kind = kinds
            .into_iter()
            .find(|kind| matches!(kind, BackupProviderKind::Notion));

        if notion_credential_kind.is_none() {
            return Ok(None);
        }

        let credentials = GetBackupCredentialsOperation {
            provider: &self.storage(),
        }
        .execute(&BackupProviderKind::Notion.into())?
        .into();

        Ok(Some(credentials))
    }

    pub fn get_command(&self, id: Uuid) -> Result<Command> {
        let command = GetCommandOperation {
            provider: &self.storage(),
        }
        .execute(&id.into())?;

        Ok(command.into())
    }

    pub fn get_workspace(&self, id: Uuid) -> Result<Workspace> {
        let workspace = GetWorkspaceOperation {
            provider: &self.storage(),
        }
        .execute(&id.into())?;

        Ok(workspace.into())
    }

    pub fn import(&self, kind: BackupProviderKind) -> Result<()> {
        let storage = self.storage();

        ImportOperation {
            backup_credentials_provider: &storage,
            upsert_commands_provider: &storage,
            upsert_workspaces_provider: &storage,
            backup_provider_builder: &NotionBackupBuilder {
                page_size: DEFAULT_BACKUP_PAGE_SIZE,
            },
            backup_provider: std::marker::PhantomData,
        }
        .execute(&kind.into())?;

        Ok(())
    }

    pub fn list_backup_credentials(&self) -> Result<Vec<BackupProviderKind>> {
        let backup_credentials = ListBackupCredentialsOperation {
            provider: &self.storage(),
        }
        .execute()?;

        Ok(backup_credentials.into_iter().map(Into::into).collect())
    }

    pub fn list_workspace_commands(
        &self,
        parameters: ListCommandsWithinWorkspaceInput,
    ) -> Result<Vec<Command>> {
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
            page_size: page_size.unwrap_or(DEFAULT_PAGE_SIZE),
            page_number: page_number.unwrap_or(FIRST_PAGE),
            program_contains: Some(program_contains),
            workspace_id: Some(&workspace_id.into()),
        })?;

        Ok(workspaces.into_iter().map(Into::into).collect())
    }

    pub fn list_workspaces(&self, parameters: ListWorkspacesInput<'_>) -> Result<Vec<Workspace>> {
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
            page_number: page_number.unwrap_or(FIRST_PAGE),
            page_size: page_size.unwrap_or(DEFAULT_PAGE_SIZE),
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

    pub fn save_backup_credentials(&self, credentials: BackupCredentials) -> Result<()> {
        let credentials = credentials.try_into()?;

        SaveBackupCredentialsOperation {
            save_provider: &self.storage(),
            backup_provider_builder: &NotionBackupBuilder {
                page_size: DEFAULT_BACKUP_PAGE_SIZE,
            },
            backup_provider: std::marker::PhantomData,
        }
        .execute(&credentials)?;

        Ok(())
    }

    fn storage(&self) -> Storage {
        Storage {
            conn: &self.database_connection,
        }
    }

    pub fn update_command(&self, data: Command) -> Result<Command> {
        let Command {
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

    pub fn update_workspace(&self, presenter: Workspace) -> Result<Workspace> {
        let Workspace { id, location, name } = presenter;

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
