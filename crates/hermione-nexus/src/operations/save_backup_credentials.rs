use crate::{
    definitions::BackupCredentials,
    services::{
        BackupService, BackupServiceBuilder, SaveBackupCredentials, StorageService,
        VerifyBackupCredentials,
    },
    Error, Result,
};
use std::marker::PhantomData;

pub struct SaveBackupCredentialsOperation<'a, S, BPB, V>
where
    S: StorageService,
    BPB: BackupServiceBuilder<V>,
    V: BackupService,
{
    save_provider: &'a S,
    backup_provider_builder: &'a BPB,
    backup_provider: PhantomData<V>,
}

pub struct SaveBackupCredentialsOperationParameters<'a, S, BPB> {
    pub save_provider: &'a S,
    pub backup_provider_builder: &'a BPB,
}

impl<'a, S, BPB, V> SaveBackupCredentialsOperation<'a, S, BPB, V>
where
    S: SaveBackupCredentials,
    BPB: BackupServiceBuilder<V>,
    V: VerifyBackupCredentials,
{
    fn build_backup_provider(&self, credentials: &BackupCredentials) -> Result<V> {
        self.backup_provider_builder
            .build_backup_provider(credentials)
    }

    pub fn execute(&self, credentials: &BackupCredentials) -> Result<()> {
        tracing::info!(operation = "Save backup credentials");

        let backup_provider = self.build_backup_provider(credentials)?;

        if !backup_provider.verify_backup_credentials()? {
            return Err(Error::Verification("Backup credentials".to_string()));
        }

        self.save_provider.save_backup_credentials(credentials)
    }

    pub fn new(parameters: SaveBackupCredentialsOperationParameters<'a, S, BPB>) -> Self {
        let SaveBackupCredentialsOperationParameters {
            save_provider,
            backup_provider_builder,
        } = parameters;

        Self {
            save_provider,
            backup_provider_builder,
            backup_provider: PhantomData,
        }
    }
}
