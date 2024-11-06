use super::GetBackupCredentialsOperation;
use crate::{
    definitions::{BackupCredentials, BackupProviderKind, Command, Workspace},
    services::{
        BackupService, BackupServiceBuilder, FilterCommandsParameters, FilterWorkspacesParameters,
        FindBackupCredentials, ListCommands, ListWorkspaces, StorageService, UpsertCommandsBackup,
        UpsertWorkspacesBackup,
    },
    Result,
};
use std::marker::PhantomData;

const BACKUP_BATCH_SIZE: u32 = 100;

pub struct ExportOperation<'a, BCP, LCP, LWP, BPB, BP>
where
    BCP: StorageService,
    LCP: StorageService,
    LWP: StorageService,
    BPB: BackupServiceBuilder<BP>,
    BP: BackupService,
{
    pub backup_credentials_provider: &'a BCP,
    pub list_commands_provider: &'a LCP,
    pub list_workspaces_provider: &'a LWP,
    pub backup_provider_builder: &'a BPB,
    pub backup_provider: PhantomData<BP>,
}

impl<'a, BCP, LCP, LWP, BPB, BP> ExportOperation<'a, BCP, LCP, LWP, BPB, BP>
where
    LCP: ListCommands,
    LWP: ListWorkspaces,
    BCP: FindBackupCredentials,
    BPB: BackupServiceBuilder<BP>,
    BP: UpsertCommandsBackup + UpsertWorkspacesBackup,
{
    fn build_backup_provider(&self, credentials: &BackupCredentials) -> Result<BP> {
        self.backup_provider_builder
            .build_backup_provider(credentials)
    }

    fn import_commands(&self, backup_provider: &BP) -> Result<()> {
        let mut page_number = 0;

        loop {
            let commands = self.list_commands(page_number)?;

            if commands.is_empty() {
                break;
            }

            backup_provider.upsert_commands_backup(commands)?;
            page_number += 1;
        }

        Ok(())
    }

    fn import_workspaces(&self, backup_provider: &BP) -> Result<()> {
        let mut page_number = 0;

        loop {
            let workspaces = self.list_workspaces(page_number)?;

            if workspaces.is_empty() {
                break;
            }

            backup_provider.upsert_workspaces_backup(workspaces)?;
            page_number += 1;
        }

        Ok(())
    }

    pub fn execute(&self, kind: &BackupProviderKind) -> Result<()> {
        tracing::info!(operation = "Export");

        let credentials = self.get_backup_credentials(kind)?;
        let backup_provider = self.build_backup_provider(&credentials)?;

        self.import_workspaces(&backup_provider)?;
        self.import_commands(&backup_provider)?;

        Ok(())
    }

    fn get_backup_credentials(&self, kind: &BackupProviderKind) -> Result<BackupCredentials> {
        GetBackupCredentialsOperation {
            provider: self.backup_credentials_provider,
        }
        .execute(kind)
    }

    fn list_commands(&self, page_number: u32) -> Result<Vec<Command>> {
        let parameters = FilterCommandsParameters {
            program_contains: None,
            page_number,
            page_size: BACKUP_BATCH_SIZE,
            workspace_id: None,
        };

        self.list_commands_provider.list_commands(parameters)
    }

    fn list_workspaces(&self, page_number: u32) -> Result<Vec<Workspace>> {
        let parameters = FilterWorkspacesParameters {
            name_contains: None,
            page_number,
            page_size: BACKUP_BATCH_SIZE,
        };

        self.list_workspaces_provider.list_workspaces(parameters)
    }
}
