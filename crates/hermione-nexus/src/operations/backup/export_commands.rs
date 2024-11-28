use crate::{
    definitions::{BackupCredentials, BackupProviderKind, Command},
    operations::GetBackupCredentialsOperation,
    services::{
        BackupCommands, BackupService, BackupServiceBuilder, FilterCommandsParameters,
        FindBackupCredentials, ListCommands, StorageService,
    },
    Result,
};
use std::marker::PhantomData;

const BACKUP_BATCH_SIZE: u32 = 100;

pub struct ExportCommandsOperation<'a, BCP, LWP, BPB, BP>
where
    BCP: StorageService,
    LWP: StorageService,
    BPB: BackupServiceBuilder<BP>,
    BP: BackupService,
{
    backup_credentials: &'a BCP,
    commands: &'a LWP,
    backup_builder: &'a BPB,
    backup: PhantomData<BP>,
}

pub struct ExportCommandsOperationParameters<'a, BCP, LWP, BPB> {
    pub backup_credentials: &'a BCP,
    pub commands: &'a LWP,
    pub backup_builder: &'a BPB,
}

impl<'a, BCP, LWP, BPB, BP> ExportCommandsOperation<'a, BCP, LWP, BPB, BP>
where
    LWP: ListCommands,
    BCP: FindBackupCredentials,
    BPB: BackupServiceBuilder<BP>,
    BP: BackupCommands,
{
    fn build_backup_provider(&self, credentials: &BackupCredentials) -> Result<BP> {
        self.backup_builder.build_backup_provider(credentials)
    }

    fn export_commands(&self, backup_provider: &BP) -> Result<()> {
        let mut page_number = 0;

        loop {
            let commands = self.list_commands(page_number)?;

            if commands.is_empty() {
                break;
            }

            backup_provider.backup_commands(commands)?;
            page_number += 1;
        }

        Ok(())
    }

    pub fn execute(&self, kind: BackupProviderKind) -> Result<()> {
        tracing::info!(operation = "Export commands");

        let credentials = self.get_backup_credentials(kind)?;
        let backup_provider = self.build_backup_provider(&credentials)?;

        self.export_commands(&backup_provider)?;

        Ok(())
    }

    fn get_backup_credentials(&self, kind: BackupProviderKind) -> Result<BackupCredentials> {
        GetBackupCredentialsOperation {
            provider: self.backup_credentials,
        }
        .execute(kind)
    }

    fn list_commands(&self, page_number: u32) -> Result<Vec<Command>> {
        let parameters = FilterCommandsParameters {
            program_contains: None,
            page_number,
            page_size: BACKUP_BATCH_SIZE,
            workspace_id: None,
        };

        self.commands.list_commands(parameters)
    }

    pub fn new(parameters: ExportCommandsOperationParameters<'a, BCP, LWP, BPB>) -> Self {
        let ExportCommandsOperationParameters {
            backup_credentials,
            commands,
            backup_builder,
        } = parameters;

        Self {
            backup_credentials,
            commands,
            backup_builder,
            backup: PhantomData,
        }
    }
}
