use crate::{
    definitions::{BackupCredentials, BackupProviderKind},
    operations::GetBackupCredentialsOperation,
    services::{
        BackupCopies, BackupCopyParameters, BackupService, BackupServiceBuilder,
        FindBackupCredentials, GetCommandsBackupCopy, GetWorkspacesBackupCopy, StorageService,
        UpsertCommands, UpsertWorkspaces,
    },
    Result,
};
use std::marker::PhantomData;

pub struct ImportOperation<'a, BCP, IUCP, IUWP, BPB, BP>
where
    BCP: StorageService,
    IUCP: StorageService,
    IUWP: StorageService,
    BPB: BackupServiceBuilder<BP>,
    BP: BackupService,
{
    pub backup_credentials_provider: &'a BCP,
    pub upsert_commands_provider: &'a IUCP,
    pub upsert_workspaces_provider: &'a IUWP,
    pub backup_provider_builder: &'a BPB,
    pub backup_provider: PhantomData<BP>,
}

impl<'a, BCP, UCP, UWP, BPB, BP> ImportOperation<'a, BCP, UCP, UWP, BPB, BP>
where
    BCP: FindBackupCredentials,
    UCP: UpsertCommands,
    UWP: UpsertWorkspaces,
    BPB: BackupServiceBuilder<BP>,
    BP: GetCommandsBackupCopy + GetWorkspacesBackupCopy,
{
    fn build_backup_provider(&self, credentials: &BackupCredentials) -> Result<BP> {
        self.backup_provider_builder
            .build_backup_provider(credentials)
    }

    fn import_commands(&self, backup_provider: &BP) -> Result<()> {
        let mut page_token = None;

        loop {
            let backup = backup_provider.get_commands_backup_copy(BackupCopyParameters {
                page_token: page_token.as_deref(),
            })?;

            let BackupCopies {
                copies: collection,
                next_page_token,
            } = backup;

            self.upsert_commands_provider.upsert_commands(collection)?;

            if next_page_token.is_none() {
                break;
            }

            page_token = next_page_token;
        }

        Ok(())
    }

    fn import_workspaces(&self, backup_provider: &BP) -> Result<()> {
        let mut page_token = None;

        loop {
            let backup = backup_provider.get_workspaces_backup_copy(BackupCopyParameters {
                page_token: page_token.as_deref(),
            })?;

            let BackupCopies {
                copies: collection,
                next_page_token,
            } = backup;

            self.upsert_workspaces_provider
                .upsert_workspaces(collection)?;

            if next_page_token.is_none() {
                break;
            }

            page_token = next_page_token;
        }

        Ok(())
    }

    pub fn execute(&self, kind: BackupProviderKind) -> Result<()> {
        tracing::info!(operation = "Import");

        let credentials = self.get_backup_credentials(kind)?;
        let backup_provider = self.build_backup_provider(&credentials)?;

        self.import_workspaces(&backup_provider)?;
        self.import_commands(&backup_provider)?;

        Ok(())
    }

    fn get_backup_credentials(&self, kind: BackupProviderKind) -> Result<BackupCredentials> {
        GetBackupCredentialsOperation {
            provider: self.backup_credentials_provider,
        }
        .execute(kind)
    }
}
