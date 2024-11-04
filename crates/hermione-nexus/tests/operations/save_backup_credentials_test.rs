use crate::solutions::{
    backup_credentials_fixture, BackupCredentialsFixtureParameters, InMemoryStorageProvider,
    MockBackupProviderBuilder, NotionBackupCredentialsFixtureParameters,
};
use hermione_nexus::{operations::SaveBackupCredentialsOperation, Error, Result};
use std::marker::PhantomData;

struct SaveBackupCredentialsOperationTestContext {
    storage: InMemoryStorageProvider,
    backup_provider_builder: MockBackupProviderBuilder,
}

fn with_context<T>(test_fn: T) -> Result<()>
where
    T: FnOnce(SaveBackupCredentialsOperationTestContext) -> Result<()>,
{
    let storage = InMemoryStorageProvider::new();
    let backup_provider_builder = MockBackupProviderBuilder::default();

    test_fn(SaveBackupCredentialsOperationTestContext {
        storage,
        backup_provider_builder,
    })
}

#[test]
fn it_saves_backup_credentials() -> Result<()> {
    with_context(|ctx| {
        let SaveBackupCredentialsOperationTestContext {
            storage,
            backup_provider_builder,
        } = ctx;

        let credentials_fixtures = backup_credentials_fixture(Default::default());

        assert_eq!(storage.count_backup_credentials()?, 0);

        SaveBackupCredentialsOperation {
            save_provider: &storage,
            backup_provider_builder: &backup_provider_builder,
            backup_provider: PhantomData,
        }
        .execute(&credentials_fixtures)?;

        assert_eq!(storage.count_backup_credentials()?, 1);

        Ok(())
    })
}

#[test]
fn it_returns_verification_error() -> Result<()> {
    with_context(|ctx| {
        let SaveBackupCredentialsOperationTestContext {
            storage,
            backup_provider_builder,
        } = ctx;

        let credentials_fixtures = backup_credentials_fixture(
            BackupCredentialsFixtureParameters::Notion(NotionBackupCredentialsFixtureParameters {
                api_key: Some("invalid_api_key".to_string()),
                ..Default::default()
            }),
        );

        assert_eq!(storage.count_backup_credentials()?, 0);

        let res = SaveBackupCredentialsOperation {
            save_provider: &storage,
            backup_provider_builder: &backup_provider_builder,
            backup_provider: PhantomData,
        }
        .execute(&credentials_fixtures);

        assert_eq!(storage.count_backup_credentials()?, 0);
        assert!(matches!(res, Err(Error::Verification(_))));

        Ok(())
    })
}
