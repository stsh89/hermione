use super::{GetBackupCredentialsOperation, GetWorkspaceOperation};
use crate::{
    definitions::{BackupCredentials, BackupProviderKind, Workspace, WorkspaceId},
    services::{BackupServiceBuilder, BackupWorkspace, FindBackupCredentials, FindWorkspace},
    Result,
};
use std::marker::PhantomData;

pub struct ExportWorkspaceOperation<'a, FBC, FW, BPB, BP> {
    find_backup_credentials: &'a FBC,
    find_workspace: &'a FW,
    backup_provider_builder: &'a BPB,
    backup_provider: PhantomData<BP>,
}

pub struct ExportWorkspaceOperationParameters<'a, FBC, FW, BPB> {
    pub find_backup_credentials: &'a FBC,
    pub find_workspace: &'a FW,
    pub backup_provider_builder: &'a BPB,
}

pub struct ExportWorkspaceParameters {
    pub workspace_id: WorkspaceId,
    pub backup_provider_kind: BackupProviderKind,
}

impl<'a, FBC, FW, BPB, BP> ExportWorkspaceOperation<'a, FBC, FW, BPB, BP>
where
    FBC: FindBackupCredentials,
    FW: FindWorkspace,
    BPB: BackupServiceBuilder<BP>,
    BP: BackupWorkspace,
{
    fn build_backup_provider(&self, credentials: BackupCredentials) -> Result<BP> {
        self.backup_provider_builder
            .build_backup_provider(&credentials)
    }

    pub fn execute(&self, parameters: ExportWorkspaceParameters) -> Result<()> {
        let ExportWorkspaceParameters {
            workspace_id,
            backup_provider_kind,
        } = parameters;

        let credentials = self.get_backup_credentials(backup_provider_kind)?;
        let workspace = self.get_workspace(workspace_id)?;
        let backup = self.build_backup_provider(credentials)?;

        backup.backup_workspace(workspace)
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

    fn get_workspace(&self, id: WorkspaceId) -> Result<Workspace> {
        GetWorkspaceOperation {
            provider: self.find_workspace,
        }
        .execute(&id)
    }

    pub fn new(parameters: ExportWorkspaceOperationParameters<'a, FBC, FW, BPB>) -> Self {
        let ExportWorkspaceOperationParameters {
            find_backup_credentials,
            find_workspace,
            backup_provider_builder,
        } = parameters;

        Self {
            find_backup_credentials,
            find_workspace,
            backup_provider_builder,
            backup_provider: PhantomData,
        }
    }
}
