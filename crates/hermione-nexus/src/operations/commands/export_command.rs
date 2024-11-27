use crate::{
    definitions::{BackupCredentials, BackupProviderKind, Command, CommandId},
    operations::{GetBackupCredentialsOperation, GetCommandOperation},
    services::{BackupCommand, BackupServiceBuilder, FindBackupCredentials, FindCommand},
    Result,
};
use std::marker::PhantomData;

pub struct ExportCommandOperation<'a, FBC, FC, BPB, BP> {
    find_backup_credentials: &'a FBC,
    find_command: &'a FC,
    backup_provider_builder: &'a BPB,
    backup_provider: PhantomData<BP>,
}

pub struct ExportCommandOperationParameters<'a, FBC, FC, BPB> {
    pub find_backup_credentials: &'a FBC,
    pub find_command: &'a FC,
    pub backup_provider_builder: &'a BPB,
}

pub struct ExportCommandParameters {
    pub command_id: CommandId,
    pub backup_provider_kind: BackupProviderKind,
}

impl<'a, FBC, FC, BPB, BP> ExportCommandOperation<'a, FBC, FC, BPB, BP>
where
    FBC: FindBackupCredentials,
    FC: FindCommand,
    BPB: BackupServiceBuilder<BP>,
    BP: BackupCommand,
{
    fn build_backup_provider(&self, credentials: BackupCredentials) -> Result<BP> {
        self.backup_provider_builder
            .build_backup_provider(&credentials)
    }

    pub fn execute(&self, parameters: ExportCommandParameters) -> Result<()> {
        let ExportCommandParameters {
            command_id,
            backup_provider_kind,
        } = parameters;

        let credentials = self.get_backup_credentials(backup_provider_kind)?;
        let command = self.get_command(command_id)?;
        let backup = self.build_backup_provider(credentials)?;

        backup.backup_command(command)
    }

    fn get_backup_credentials(
        &self,
        backup_provider_kind: BackupProviderKind,
    ) -> Result<BackupCredentials> {
        GetBackupCredentialsOperation {
            provider: self.find_backup_credentials,
        }
        .execute(backup_provider_kind)
    }

    fn get_command(&self, id: CommandId) -> Result<Command> {
        GetCommandOperation {
            provider: self.find_command,
        }
        .execute(&id)
    }

    pub fn new(parameters: ExportCommandOperationParameters<'a, FBC, FC, BPB>) -> Self {
        let ExportCommandOperationParameters {
            find_backup_credentials,
            find_command,
            backup_provider_builder,
        } = parameters;

        Self {
            find_backup_credentials,
            find_command,
            backup_provider_builder,
            backup_provider: PhantomData,
        }
    }
}
