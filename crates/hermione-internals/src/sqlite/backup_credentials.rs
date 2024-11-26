use eyre::eyre;
use hermione_nexus::{
    definitions::{BackupCredentials, BackupProviderKind, NotionBackupCredentialsParameters},
    Error,
};
use rusqlite::{named_params, params, Connection, OptionalExtension, Result};
use serde::{Deserialize, Serialize};

const NOTION_BACKUP_CREDENTIALS_ID: &str = "Notion";

pub struct BackupCredentialsRecord {
    pub id: String,
    pub secrets: String,
}

enum BackupCredentialsId {
    Notion,
}

#[derive(Serialize, Deserialize)]
pub struct NotionBackupSecrets {
    pub api_key: String,
    pub commands_database_id: String,
    pub workspaces_database_id: String,
}

pub fn create_backup_credentials_table_if_not_exists(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS backup_credentials (
            id TEXT PRIMARY KEY,
            secrets TEXT NOT NULL
        )",
        (),
    )?;

    Ok(())
}

pub fn find_backup_credentials(
    conn: &Connection,
    kind: BackupProviderKind,
) -> Result<Option<BackupCredentialsRecord>> {
    let id = match kind {
        BackupProviderKind::Notion => NOTION_BACKUP_CREDENTIALS_ID,
    };

    conn.prepare(
        "SELECT
            id,
            secrets
        FROM backup_credentials
        WHERE id = ?1",
    )?
    .query_row(params![id], |row| {
        Ok(BackupCredentialsRecord {
            id: row.get(0)?,
            secrets: row.get(1)?,
        })
    })
    .optional()
}

pub fn delete_backup_credentials(conn: &Connection, kind: BackupProviderKind) -> Result<usize> {
    let id = match kind {
        BackupProviderKind::Notion => NOTION_BACKUP_CREDENTIALS_ID,
    };

    conn.prepare("DELETE FROM backup_credentials WHERE id = ?1")?
        .execute(params![id])
}

pub fn insert_backup_credentials(
    conn: &Connection,
    record: BackupCredentialsRecord,
) -> Result<usize> {
    let BackupCredentialsRecord { id, secrets } = record;

    conn.prepare(
        "INSERT INTO backup_credentials (
            id,
            secrets
        ) VALUES (:id, :secrets)",
    )?
    .execute(named_params![
        ":id": id,
        ":secrets": secrets,
    ])
}

pub fn list_backup_credentials(conn: &Connection) -> Result<Vec<BackupCredentialsRecord>> {
    let mut statement = conn.prepare(
        "SELECT
            id,
            secrets
        FROM backup_credentials
        ORDER BY id ASC",
    )?;

    let records = statement
        .query_map([], |row| {
            Ok(BackupCredentialsRecord {
                id: row.get(0)?,
                secrets: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(records)
}

pub fn update_backup_credentials(
    conn: &Connection,
    record: BackupCredentialsRecord,
) -> Result<usize> {
    let BackupCredentialsRecord { id, secrets } = record;

    conn.prepare(
        "UPDATE backup_credentials
        SET
            secrets = :secrets
        WHERE id = :id",
    )?
    .execute(named_params![
        ":id": id,
        ":secrets": secrets
    ])
}

impl TryFrom<&str> for BackupCredentialsId {
    type Error = Error;

    fn try_from(value: &str) -> hermione_nexus::Result<Self> {
        let id = match value {
            NOTION_BACKUP_CREDENTIALS_ID => BackupCredentialsId::Notion,
            _ => {
                return Err(Error::storage(
                    eyre!("Unexpected backup credentials id: {}", value)
                        .wrap_err("Corrupted storage data"),
                ));
            }
        };

        Ok(id)
    }
}

impl TryFrom<&BackupCredentials> for BackupCredentialsRecord {
    type Error = hermione_nexus::Error;

    fn try_from(value: &BackupCredentials) -> hermione_nexus::Result<Self> {
        match value {
            BackupCredentials::Notion(notion_backup_credentials) => Ok(BackupCredentialsRecord {
                id: NOTION_BACKUP_CREDENTIALS_ID.to_string(),
                secrets: serde_json::to_string(&NotionBackupSecrets {
                    api_key: notion_backup_credentials.api_key().to_string(),
                    commands_database_id: notion_backup_credentials
                        .commands_database_id()
                        .to_string(),
                    workspaces_database_id: notion_backup_credentials
                        .workspaces_database_id()
                        .to_string(),
                })
                .map_err(|err| {
                    eyre::Error::new(err)
                        .wrap_err("Failed to convert backup credentials into internal format")
                })
                .map_err(Error::storage)?,
            }),
        }
    }
}

impl TryFrom<BackupCredentials> for BackupCredentialsRecord {
    type Error = hermione_nexus::Error;

    fn try_from(value: BackupCredentials) -> hermione_nexus::Result<Self> {
        TryFrom::try_from(&value)
    }
}

impl TryFrom<BackupCredentialsRecord> for BackupCredentials {
    type Error = Error;

    fn try_from(value: BackupCredentialsRecord) -> hermione_nexus::Result<Self> {
        let BackupCredentialsRecord { id, secrets } = value;

        match BackupCredentialsId::try_from(id.as_str())? {
            BackupCredentialsId::Notion => {
                let secrets: NotionBackupSecrets = serde_json::from_str(&secrets)
                    .map_err(|err| eyre::Error::new(err).wrap_err("Corrupted storage data"))
                    .map_err(Error::storage)?;

                let NotionBackupSecrets {
                    api_key,
                    commands_database_id,
                    workspaces_database_id,
                } = secrets;

                Ok(BackupCredentials::notion(
                    NotionBackupCredentialsParameters {
                        api_key,
                        commands_database_id,
                        workspaces_database_id,
                    },
                ))
            }
        }
    }
}
