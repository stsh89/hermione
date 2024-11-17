use crate::{
    definitions::{BackupCredentials, BackupProviderKind},
    services::{
        BackupService, BackupServiceBuilder, FindBackupCredentials, ListCommandsBackup,
        StorageService, UpsertCommands,
    },
    Result,
};
use std::marker::PhantomData;
use super::GetBackupCredentialsOperation;

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
    BP: ListCommandsBackup,
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
        .execute(&backup_provider_kind)
    }

    fn import_commands(&self, backup_provider: BP) -> Result<()> {
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
