use chrono::{DateTime, Utc};
use hermione_core::{
    entities::command::{Entity, LoadParameters, Name, Program, ScopedId},
    operations::workspaces::commands::{create, delete, get, list, track_execution_time, update},
    Id, Result,
};
use hermione_json::collections::Client as InnerClient;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Record {
    id: Uuid,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execute_time: Option<DateTime<Utc>>,

    name: String,
    program: String,
    workspace_id: Uuid,
}

pub struct Client {
    inner: InnerClient,
}

impl Client {
    pub fn new(path: PathBuf) -> eyre::Result<Self> {
        let manager = InnerClient::new(path)?;

        Ok(Self { inner: manager })
    }

    pub fn read(&self) -> eyre::Result<Vec<Record>> {
        let records = self.inner.read_collection()?;

        Ok(records)
    }

    pub fn write(&self, records: Vec<Record>) -> eyre::Result<()> {
        self.inner.write_collection(records)?;

        Ok(())
    }
}

impl create::Create for Client {
    fn create(&self, mut entity: Entity) -> Result<Entity> {
        let mut records = self.read()?;
        let id = Uuid::new_v4();
        entity.set_id(Id::new(id))?;

        let record = Record::from_entity(&entity)?;
        records.push(record);

        self.write(records)?;

        Ok(entity)
    }
}

impl delete::Delete for Client {
    fn delete(&self, id: ScopedId) -> Result<()> {
        let ScopedId { workspace_id, id } = id;
        let mut records: Vec<Record> = self.read()?;

        records.retain(|record| !(record.id == *id && record.workspace_id == *workspace_id));

        self.write(records)?;

        Ok(())
    }
}

impl get::Get for Client {
    fn get(&self, id: ScopedId) -> Result<Entity> {
        let ScopedId { workspace_id, id } = id;
        let records: Vec<Record> = self.read()?;

        let record = records
            .into_iter()
            .find(|record| record.id == *id && record.workspace_id == *workspace_id)
            .ok_or_else(|| eyre::eyre!("Workspace with id {} not found", id))?;

        let entity = record.load_entity();

        Ok(entity)
    }
}

impl list::List for Client {
    fn list(&self, workspace_id: Id) -> Result<Vec<Entity>> {
        let mut records: Vec<Record> = self.read()?;
        records.sort_unstable_by(|a, b| a.last_execute_time.cmp(&b.last_execute_time).reverse());

        let entities = records
            .into_iter()
            .filter(|record| record.workspace_id == *workspace_id)
            .map(Record::load_entity)
            .collect();

        Ok(entities)
    }
}

impl track_execution_time::Track for Client {
    fn track(&self, entity: Entity) -> Result<Entity> {
        let Some(id) = entity.id() else {
            return Err(
                eyre::eyre!("Attempt to track access time for workspace without id").into(),
            );
        };

        let mut records: Vec<Record> = self.read()?;

        let record = records
            .iter_mut()
            .find(|record| record.id == *id && record.workspace_id == *entity.workspace_id())
            .ok_or(eyre::eyre!("Workspace with id {} not found", id,))?;

        record.last_execute_time = Some(Utc::now());

        self.write(records)?;

        use get::Get;
        self.get(ScopedId {
            workspace_id: entity.workspace_id(),
            id,
        })
    }
}

impl update::Update for Client {
    fn update(&self, entity: Entity) -> Result<Entity> {
        let Some(id) = entity.id() else {
            return Err(eyre::eyre!("Attemp to update workspace without id").into());
        };

        let mut records: Vec<Record> = self.read()?;

        let record = records
            .iter_mut()
            .find(|record| record.id == *id && record.workspace_id == *entity.workspace_id())
            .ok_or_else(|| eyre::eyre!("Workspace with id {}not found", id))?;

        record.name = entity.name().to_string();
        record.program = entity.program().to_string();

        self.write(records)?;

        Ok(entity)
    }
}

impl Record {
    fn from_entity(entity: &Entity) -> Result<Self> {
        Ok(Self {
            id: *entity.id().ok_or(eyre::eyre!("Record without id"))?,
            last_execute_time: entity.last_execute_time().map(From::from),
            program: entity.program().to_string(),
            name: entity.name().to_string(),
            workspace_id: *entity.workspace_id(),
        })
    }

    fn load_entity(self) -> Entity {
        Entity::load(LoadParameters {
            id: Id::new(self.id),
            last_execute_time: self.last_execute_time.map(From::from),
            program: Program::new(self.program),
            name: Name::new(self.name),
            workspace_id: Id::new(self.workspace_id),
        })
    }
}
