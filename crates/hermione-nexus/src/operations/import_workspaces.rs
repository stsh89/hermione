use crate::{
    services::{
        BackupService, BackupServiceBuilder, FindBackupCredentials, ListCommandsBackup,
        ListWorkspacesBackup, StorageService, UpsertWorkspaces,
    },
    Result,
};
use std::marker::PhantomData;

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
    pub fn execute(&self) -> Result<()> {
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
