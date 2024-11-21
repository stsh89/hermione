use crate::Result;
use hermione_drive::{
    Clipboard, NotionBackupBuilder, ServiceFactory, Storage, System, SystemParameters,
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
use std::num::NonZeroU32;
use uuid::Uuid;

pub const FIRST_PAGE: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(1) };
pub const DEFAULT_PAGE_SIZE: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(43) };

pub struct Coordinator {
    pub service_factory: ServiceFactory,
}

pub struct Workspace {
    pub id: Uuid,
    pub location: String,
    pub name: String,
}

pub struct Command {
    pub workspace_id: Uuid,
    pub id: Uuid,
    pub name: String,
    pub program: String,
}

#[derive(Clone)]
pub enum BackupProviderKind {
    Notion,
}

pub enum BackupCredentials {
    Notion(NotionBackupCredentials),
}

pub struct NotionBackupCredentials {
    pub api_key: String,
    pub commands_database_id: String,
    pub workspaces_database_id: String,
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
    fn notion_backup_builder(&self) -> NotionBackupBuilder {
        NotionBackupBuilder::default()
    }

    fn clipboard(&self) -> Clipboard {
        self.service_factory.clipboard()
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
        let system = self.system(SystemParameters {
            no_exit,
            working_directory,
        });

        ExecuteCommandOperation {
            find_command_provider: &storage,
            system_provider: &system,
            track_command_provider: &storage,
            track_workspace_provider: &storage,
        }
        .execute(&command_id.into())?;

        Ok(())
    }

    pub fn export(&self, kind: BackupProviderKind) -> Result<()> {
        let storage = self.storage();
        let backup_provider = self.notion_backup_builder();

        ExportOperation {
            backup_credentials_provider: &storage,
            list_commands_provider: &storage,
            list_workspaces_provider: &storage,
            backup_provider_builder: &backup_provider,
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
        let backup_builder = self.notion_backup_builder();

        ImportOperation {
            backup_credentials_provider: &storage,
            upsert_commands_provider: &storage,
            upsert_workspaces_provider: &storage,
            backup_provider_builder: &backup_builder,
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
        tracing::warn!("Open Windows terminal: {}", working_directory);

        // let working_directory = if working_directory.is_empty() {
        //     None
        // } else {
        //     Some(working_directory)
        // };

        // powershell::open_windows_terminal(
        //     &self.powershell_process,
        //     Some(PowerShellParameters {
        //         command: None,
        //         no_exit: false,
        //         working_directory,
        //     }),
        // )?;

        Ok(())
    }

    pub fn save_backup_credentials(&self, credentials: BackupCredentials) -> Result<()> {
        let credentials = credentials.try_into()?;
        let backup_provider = self.notion_backup_builder();

        SaveBackupCredentialsOperation {
            save_provider: &self.storage(),
            backup_provider_builder: &backup_provider,
            backup_provider: std::marker::PhantomData,
        }
        .execute(&credentials)?;

        Ok(())
    }

    fn storage(&self) -> Storage {
        self.service_factory.storage()
    }

    fn system<'a>(&'a self, paramters: SystemParameters<'a>) -> System {
        self.service_factory.system(paramters)
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

mod convert {
    mod core {
        pub use hermione_nexus::{
            definitions::{
                BackupCredentials, BackupProviderKind, Command, CommandParameters,
                NotionBackupCredentialsParameters, Workspace, WorkspaceParameters,
            },
            Error, Result,
        };
    }
    use super::{
        BackupCredentials, BackupProviderKind, Command, NotionBackupCredentials, Workspace,
    };

    impl From<BackupProviderKind> for core::BackupProviderKind {
        fn from(value: BackupProviderKind) -> Self {
            match value {
                BackupProviderKind::Notion => core::BackupProviderKind::Notion,
            }
        }
    }

    impl TryFrom<Command> for core::Command {
        type Error = hermione_nexus::Error;

        fn try_from(value: Command) -> hermione_nexus::Result<Self> {
            let Command {
                id,
                name,
                program,
                workspace_id,
            } = value;

            let command = core::Command::new(core::CommandParameters {
                id,
                name,
                last_execute_time: None,
                program,
                workspace_id: workspace_id.into(),
            })?;

            Ok(command)
        }
    }

    impl TryFrom<BackupCredentials> for core::BackupCredentials {
        type Error = core::Error;

        fn try_from(value: BackupCredentials) -> core::Result<Self> {
            let credentials = match value {
                BackupCredentials::Notion(presenter) => {
                    let NotionBackupCredentials {
                        api_key,
                        workspaces_database_id,
                        commands_database_id,
                    } = presenter;

                    core::BackupCredentials::notion(core::NotionBackupCredentialsParameters {
                        api_key,
                        commands_database_id,
                        workspaces_database_id,
                    })
                }
            };

            Ok(credentials)
        }
    }

    impl TryFrom<Workspace> for core::Workspace {
        type Error = core::Error;

        fn try_from(value: Workspace) -> core::Result<Self> {
            let Workspace { id, location, name } = value;

            let workspace = core::Workspace::new(core::WorkspaceParameters {
                id,
                name,
                location: Some(location),
                last_access_time: None,
            })?;

            Ok(workspace)
        }
    }

    impl From<core::BackupCredentials> for BackupProviderKind {
        fn from(value: core::BackupCredentials) -> Self {
            match value {
                core::BackupCredentials::Notion(_) => BackupProviderKind::Notion,
            }
        }
    }

    impl From<core::BackupCredentials> for NotionBackupCredentials {
        fn from(value: core::BackupCredentials) -> Self {
            match value {
                core::BackupCredentials::Notion(credentials) => NotionBackupCredentials {
                    api_key: credentials.api_key().to_string(),
                    workspaces_database_id: credentials.workspaces_database_id().to_string(),
                    commands_database_id: credentials.commands_database_id().to_string(),
                },
            }
        }
    }

    impl From<core::Command> for Command {
        fn from(command: core::Command) -> Self {
            Self {
                id: **command.id(),
                name: command.name().to_string(),
                program: command.program().to_string(),
                workspace_id: **command.workspace_id(),
            }
        }
    }

    impl From<core::Workspace> for Workspace {
        fn from(workspace: core::Workspace) -> Self {
            Self {
                id: **workspace.id(),
                location: workspace.location().unwrap_or_default().into(),
                name: workspace.name().to_string(),
            }
        }
    }
}
