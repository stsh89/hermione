use crate::{
    definitions::{BackupCredentials, BackupProviderKind, Workspace},
    operations::GetBackupCredentialsOperation,
    services::{
        BackupCommands, BackupService, BackupServiceBuilder, BackupWorkspaces,
        FilterWorkspacesParameters, FindBackupCredentials, ListWorkspaces, StorageService,
    },
    Result,
};
use std::marker::PhantomData;

const BACKUP_BATCH_SIZE: u32 = 100;

pub struct ExportWorkspacesOperation<'a, BCP, LWP, BPB, BP>
where
    BCP: StorageService,
    LWP: StorageService,
    BPB: BackupServiceBuilder<BP>,
    BP: BackupService,
{
    backup_credentials: &'a BCP,
    workspaces: &'a LWP,
    backup_builder: &'a BPB,
    backup: PhantomData<BP>,
}

pub struct ExportWorkspacesOperationParameters<'a, BCP, LWP, BPB> {
    pub backup_credentials: &'a BCP,
    pub workspaces: &'a LWP,
    pub backup_builder: &'a BPB,
}

impl<'a, BCP, LWP, BPB, BP> ExportWorkspacesOperation<'a, BCP, LWP, BPB, BP>
where
    LWP: ListWorkspaces,
    BCP: FindBackupCredentials,
    BPB: BackupServiceBuilder<BP>,
    BP: BackupCommands + BackupWorkspaces,
{
    fn build_backup_provider(&self, credentials: &BackupCredentials) -> Result<BP> {
        self.backup_builder.build_backup_provider(credentials)
    }

    fn export_workspaces(&self, backup_provider: &BP) -> Result<()> {
        let mut page_number = 0;

        loop {
            let workspaces = self.list_workspaces(page_number)?;

            if workspaces.is_empty() {
                break;
            }

            backup_provider.backup_workspaces(workspaces)?;
            page_number += 1;
        }

        Ok(())
    }

    pub fn execute(&self, kind: BackupProviderKind) -> Result<()> {
        tracing::info!(operation = "Export workspaces");

        let credentials = self.get_backup_credentials(kind)?;
        let backup_provider = self.build_backup_provider(&credentials)?;

        self.export_workspaces(&backup_provider)?;

        Ok(())
    }

    fn get_backup_credentials(&self, kind: BackupProviderKind) -> Result<BackupCredentials> {
        GetBackupCredentialsOperation {
            provider: self.backup_credentials,
        }
        .execute(kind)
    }

    fn list_workspaces(&self, page_number: u32) -> Result<Vec<Workspace>> {
        let parameters = FilterWorkspacesParameters {
            name_contains: None,
            page_number,
            page_size: BACKUP_BATCH_SIZE,
        };

        self.workspaces.list_workspaces(parameters)
    }

    pub fn new(parameters: ExportWorkspacesOperationParameters<'a, BCP, LWP, BPB>) -> Self {
        let ExportWorkspacesOperationParameters {
            backup_credentials,
            workspaces,
            backup_builder,
        } = parameters;

        Self {
            backup_credentials,
            workspaces,
            backup_builder,
            backup: PhantomData,
        }
    }
}
