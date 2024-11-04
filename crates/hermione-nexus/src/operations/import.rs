use crate::{
    definitions::{BackupCredentials, BackupProviderKind},
    operations::GetBackupCredentialsOperation,
    services::{
        BackupProviderBuilder, FindBackupCredentials, ListCommandsBackup, ListWorkspacesBackup,
        UpsertCommands, UpsertWorkspaces,
    },
    Result,
};
use std::marker::PhantomData;

pub struct ImportOperation<'a, BCP, IUCP, IUWP, BPB, BP> {
    pub backup_credentials_provider: &'a BCP,
    pub upsert_commands_provider: &'a IUCP,
    pub upsert_workspaces_provider: &'a IUWP,
    pub backup_provider_builder: &'a BPB,
    pub phantom_backup_provider_builder: PhantomData<BP>,
}

impl<'a, BCP, UCP, UWP, BPB, BP> ImportOperation<'a, BCP, UCP, UWP, BPB, BP>
where
    BCP: FindBackupCredentials,
    UCP: UpsertCommands,
    UWP: UpsertWorkspaces,
    BPB: BackupProviderBuilder<BP>,
    BP: ListCommandsBackup + ListWorkspacesBackup,
{
    fn backup_provider(&self, credentials: &BackupCredentials) -> Result<BP> {
        self.backup_provider_builder
            .build_backup_provider(credentials)
    }

    fn backup_credentials(&self, kind: &BackupProviderKind) -> Result<BackupCredentials> {
        GetBackupCredentialsOperation {
            provider: self.backup_credentials_provider,
        }
        .execute(kind)
    }

    fn import_commands(&self, backup_provider: &BP) -> Result<()> {
        let mut page_id = None;

        while let Some((commands, next_page_id)) =
            backup_provider.list_commands_backup(page_id.as_deref())?
        {
            self.upsert_commands_provider.upsert_commands(commands)?;

            if next_page_id.is_none() {
                break;
            }

            page_id = next_page_id;
        }

        Ok(())
    }

    fn import_workspaces(&self, backup_provider: &BP) -> Result<()> {
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

    pub fn execute(&self, kind: &BackupProviderKind) -> Result<()> {
        tracing::info!(operation = "Import");

        let credentials = self.backup_credentials(kind)?;
        let backup_provider = self.backup_provider(&credentials)?;

        self.import_workspaces(&backup_provider)?;
        self.import_commands(&backup_provider)?;

        Ok(())
    }
}
