use chrono::{DateTime, Utc};
use hermione_core::{
    entities::workspace::{Entity, LoadParameters, Location, Name},
    operations::workspaces::{create, delete, get, list, track_access_time, update},
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
    last_access_time: Option<DateTime<Utc>>,

    location: String,
    name: String,
}

pub struct Client {
    pub inner: InnerClient,
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
    fn delete(&self, id: Id) -> Result<()> {
        let mut record = self.read()?;

        record.retain(|record| record.id != *id);

        self.write(record)?;

        Ok(())
    }
}

impl get::Get for Client {
    fn get(&self, id: Id) -> Result<Entity> {
        let records = self.read()?;

        let record = records
            .into_iter()
            .find(|record| record.id == *id)
            .ok_or_else(|| eyre::eyre!("Workspace with id {} not found", id))?;

        let entity = record.load_entity();

        Ok(entity)
    }
}

impl list::List for Client {
    fn list(&self) -> Result<Vec<Entity>> {
        let mut records = self.read()?;
        records.sort_unstable_by(|a, b| a.last_access_time.cmp(&b.last_access_time).reverse());

        let entities = records.into_iter().map(Record::load_entity).collect();

        Ok(entities)
    }
}

impl track_access_time::Track for Client {
    fn track(&self, entity: Entity) -> Result<Entity> {
        let Some(id) = entity.id() else {
            return Err(
                eyre::eyre!("Attempt to track access time for workspace without id").into(),
            );
        };

        let mut records = self.read()?;
        let record = records
            .iter_mut()
            .find(|record| record.id == *id)
            .ok_or(eyre::eyre!("Workspace with id {} not found", id,))?;

        record.last_access_time = Some(Utc::now());

        self.write(records)?;

        use get::Get;
        self.get(id)
    }
}

impl update::Update for Client {
    fn update(&self, entity: Entity) -> Result<Entity> {
        let Some(id) = entity.id() else {
            return Err(eyre::eyre!("Attemp to update workspace without id").into());
        };

        let mut records = self.read()?;
        let record = records
            .iter_mut()
            .find(|record| record.id == *id)
            .ok_or_else(|| eyre::eyre!("Workspace with id {}not found", id))?;

        record.name = entity.name().to_string();
        record.location = entity
            .location()
            .map(ToString::to_string)
            .unwrap_or_default();

        self.write(records)?;

        Ok(entity)
    }
}

impl Record {
    fn from_entity(entity: &Entity) -> Result<Self> {
        Ok(Self {
            id: *entity.id().ok_or(eyre::eyre!("Record without id"))?,
            last_access_time: entity.last_access_time().map(From::from),
            location: entity
                .location()
                .map(ToString::to_string)
                .unwrap_or_default(),
            name: entity.name().to_string(),
        })
    }

    fn load_entity(self) -> Entity {
        Entity::load(LoadParameters {
            id: Id::new(self.id),
            last_access_time: self.last_access_time.map(From::from),
            location: Some(Location::new(self.location)),
            name: Name::new(self.name),
        })
    }
}
