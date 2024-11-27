use crate::{
    definitions::{BackupCredentials, BackupProviderKind},
    operations::GetBackupCredentialsOperation,
    services::{
        BackupCopies, BackupCopyParameters, BackupService, BackupServiceBuilder,
        FindBackupCredentials, GetCommandsBackupCopy, StorageService, UpsertCommands,
    },
    Result,
};
use std::marker::PhantomData;

pub struct ImportCommandsOperation<'a, BCP, UCP, BPB, BP>
where
    BCP: StorageService,
    UCP: StorageService,
    BPB: BackupServiceBuilder<BP>,
    BP: BackupService,
{
    backup_credentials_provider: &'a BCP,
    upsert_commands_provider: &'a UCP,
    backup_provider_builder: &'a BPB,
    backup_provider: PhantomData<BP>,
}

pub struct ImportCommandsOperationParameters<'a, BCP, UCP, BPB> {
    pub backup_credentials_provider: &'a BCP,
    pub upsert_commands_provider: &'a UCP,
    pub backup_provider_builder: &'a BPB,
}

impl<'a, BCP, UCP, BPB, BP> ImportCommandsOperation<'a, BCP, UCP, BPB, BP>
where
    BCP: FindBackupCredentials,
    UCP: UpsertCommands,
    BPB: BackupServiceBuilder<BP>,
    BP: GetCommandsBackupCopy,
{
    fn build_backup_provider(&self, credentials: BackupCredentials) -> Result<BP> {
        self.backup_provider_builder
            .build_backup_provider(&credentials)
    }

    pub fn execute(&self, backup_provider_kind: BackupProviderKind) -> Result<()> {
        tracing::info!(operation = "Import commands");

        let credentials = self.get_backup_credentials(backup_provider_kind)?;
        let backup_provider = self.build_backup_provider(credentials)?;

        self.import_commands(backup_provider)?;

        Ok(())
    }

    fn get_backup_credentials(
        &self,
        backup_provider_kind: BackupProviderKind,
    ) -> Result<BackupCredentials> {
        GetBackupCredentialsOperation {
            provider: self.backup_credentials_provider,
        }
        .execute(backup_provider_kind)
    }

    fn import_commands(&self, backup_provider: BP) -> Result<()> {
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

    pub fn new(parameters: ImportCommandsOperationParameters<'a, BCP, UCP, BPB>) -> Self {
        let ImportCommandsOperationParameters {
            backup_credentials_provider,
            upsert_commands_provider,
            backup_provider_builder,
        } = parameters;

        ImportCommandsOperation {
            backup_credentials_provider,
            upsert_commands_provider,
            backup_provider_builder,
            backup_provider: PhantomData,
        }
    }
}
