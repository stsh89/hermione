use super::GetBackupCredentialsOperation;
use crate::{
    definitions::{BackupCredentials, BackupProviderKind},
    services::{
        BackupProviderBuilder, FilterWorkspacesParameters, FindBackupCredentials, ListCommands,
        ListWorkspaces, UpsertCommandsBackup, UpsertWorkspacesBackup,
    },
    Result,
};
use std::marker::PhantomData;

const BACKUP_BATCH_SIZE: u32 = 100;

pub struct ExportOperation<'a, BCP, LCP, LWP, BPB, BP> {
    pub backup_credentials_provider: &'a BCP,
    pub list_commands_provider: &'a LCP,
    pub list_workspaces_provider: &'a LWP,
    pub backup_provider_builder: &'a BPB,
    pub phantom_backup_provider_builder: PhantomData<BP>,
}

impl<'a, BCP, LCP, LWP, BPB, BP> ExportOperation<'a, BCP, LCP, LWP, BPB, BP>
where
    LCP: ListCommands,
    LWP: ListWorkspaces,
    BCP: FindBackupCredentials,
    BPB: BackupProviderBuilder<BP>,
    BP: UpsertCommandsBackup + UpsertWorkspacesBackup,
{
    fn backup_credentials(&self, kind: &BackupProviderKind) -> Result<BackupCredentials> {
        GetBackupCredentialsOperation {
            provider: self.backup_credentials_provider,
        }
        .execute(kind)
    }

    fn backup_provider(&self, credentials: &BackupCredentials) -> Result<BP> {
        self.backup_provider_builder
            .build_backup_provider(credentials)
    }

    fn import_commands(&self, backup_provider: &BP) -> Result<()> {
        let mut page_number = 1;

        loop {
            let commands = self.list_commands_provider.list_commands(
                crate::services::FilterCommandsParameters {
                    program_contains: None,
                    page_number,
                    page_size: BACKUP_BATCH_SIZE,
                    workspace_id: None,
                },
            )?;

            if commands.is_empty() {
                break;
            }

            backup_provider.upsert_commands_backup(commands)?;
            page_number += 1;
        }

        Ok(())
    }

    fn import_workspaces(&self, backup_provider: &BP) -> Result<()> {
        let mut page_number = 1;

        loop {
            let workspaces =
                self.list_workspaces_provider
                    .list_workspaces(FilterWorkspacesParameters {
                        name_contains: None,
                        page_number,
                        page_size: BACKUP_BATCH_SIZE,
                    })?;

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

        let credentials = self.backup_credentials(kind)?;
        let backup_provider = self.backup_provider(&credentials)?;

        self.import_workspaces(&backup_provider)?;
        self.import_commands(&backup_provider)?;

        Ok(())
    }
}
