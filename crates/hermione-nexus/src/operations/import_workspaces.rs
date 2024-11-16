use crate::{
    definitions::{BackupCredentials, BackupProviderKind},
    services::{
        BackupService, BackupServiceBuilder, FindBackupCredentials, ListCommandsBackup,
        ListWorkspacesBackup, StorageService, UpsertWorkspaces,
    },
    Result,
};
use std::marker::PhantomData;

use super::GetBackupCredentialsOperation;

pub struct ImportWorkspacesOperation<'a, BCP, UWP, BPB, BP>
where
    BCP: StorageService,
    UWP: StorageService,
    BPB: BackupServiceBuilder<BP>,
    BP: BackupService,
{
    backup_credentials_provider: &'a BCP,
    upsert_workspaces_provider: &'a UWP,
    backup_provider_builder: &'a BPB,
    backup_provider: PhantomData<BP>,
}

pub struct ImportWorkspacesOperationParameters<'a, BCP, IUWP, BPB> {
    pub backup_credentials_provider: &'a BCP,
    pub upsert_workspaces_provider: &'a IUWP,
    pub backup_provider_builder: &'a BPB,
}

impl<'a, BCP, UWP, BPB, BP> ImportWorkspacesOperation<'a, BCP, UWP, BPB, BP>
where
    BCP: FindBackupCredentials,
    UWP: UpsertWorkspaces,
    BPB: BackupServiceBuilder<BP>,
    BP: ListCommandsBackup + ListWorkspacesBackup,
{
    fn build_backup_provider(&self, credentials: BackupCredentials) -> Result<BP> {
        self.backup_provider_builder
            .build_backup_provider(&credentials)
    }

    pub fn execute(&self, backup_provider_kind: BackupProviderKind) -> Result<()> {
        tracing::info!(operation = "Import workspaces");

        let credentials = self.get_backup_credentials(backup_provider_kind)?;
        let backup_provider = self.build_backup_provider(credentials)?;

        self.import_workspaces(backup_provider)?;

        Ok(())
    }

    fn get_backup_credentials(
        &self,
        backup_provider_kind: BackupProviderKind,
    ) -> Result<BackupCredentials> {
        GetBackupCredentialsOperation {
            provider: self.backup_credentials_provider,
        }
        .execute(&backup_provider_kind)
    }

    fn import_workspaces(&self, backup_provider: BP) -> Result<()> {
        let mut page_id = None;

        while let Some((workspaces, next_page_id)) =
            backup_provider.list_workspaces_backup(page_id.as_deref())?
        {
            self.upsert_workspaces_provider
                .upsert_workspaces(workspaces)?;

            if next_page_id.is_none() {
                break;
            }

            page_id = next_page_id;
        }

        Ok(())
    }

    pub fn new(parameters: ImportWorkspacesOperationParameters<'a, BCP, UWP, BPB>) -> Self {
        let ImportWorkspacesOperationParameters {
            backup_credentials_provider,
            upsert_workspaces_provider,
            backup_provider_builder,
        } = parameters;

        ImportWorkspacesOperation {
            backup_credentials_provider,
            upsert_workspaces_provider,
            backup_provider_builder,
            backup_provider: PhantomData,
        }
    }
}
