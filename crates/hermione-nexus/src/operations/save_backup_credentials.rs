use crate::{
    definitions::BackupCredentials,
    services::{BackupProviderBuilder, SaveBackupCredentials, VerifyBackupCredentials},
    Error, Result,
};
use std::marker::PhantomData;

pub struct SaveBackupCredentialsOperation<'a, S, BPB, V> {
    pub save_provider: &'a S,
    pub backup_provider_builder: &'a BPB,
    pub backup_provider_builder_phantom: PhantomData<V>,
}

impl<'a, S, BPB, V> SaveBackupCredentialsOperation<'a, S, BPB, V>
where
    S: SaveBackupCredentials,
    BPB: BackupProviderBuilder<V>,
    V: VerifyBackupCredentials,
{
    fn backup_provider(&self, credentials: &BackupCredentials) -> Result<V> {
        self.backup_provider_builder
            .build_backup_provider(credentials)
    }

    pub fn execute(&self, credentials: &BackupCredentials) -> Result<()> {
        tracing::info!(operation = "Save backup credentials");

        let backup_provider = self.backup_provider(credentials)?;

        if !backup_provider.verify_backup_credentials()? {
            return Err(Error::Verification("Backup credentials".to_string()));
        }

        self.save_provider.save_backup_credentials(credentials)
    }
}
