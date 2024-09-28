use crate::impls::json;
use hermione_memories::{
    operations::workspaces::{create, delete, get, list, track_access_time, update},
    types::{
        workspace::{Entity, LoadParameters, Location, Name},
        Error, Id,
    },
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Record {
    id: Uuid,

    #[serde(skip_serializing_if = "Option::is_none")]
    last_access_time: Option<chrono::DateTime<chrono::Utc>>,

    location: String,
    name: String,
}

impl create::Create for json::Client {
    fn create(&self, mut entity: Entity) -> Result<Entity, Error> {
        let mut records: Vec<Record> = self.read()?;
        let id = Uuid::new_v4();
        entity.set_id(Id::new(id))?;

        let record = Record::from_entity(&entity)?;
        records.push(record);

        self.save(records)?;

        Ok(entity)
    }
}

impl delete::Delete for json::Client {
    fn delete(&self, id: Id) -> Result<(), Error> {
        let mut record: Vec<Record> = self.read()?;

        record.retain(|record| record.id != *id);

        self.save(record)?;

        Ok(())
    }
}

impl get::Get for json::Client {
    fn get(&self, id: Id) -> Result<Entity, Error> {
        let records: Vec<Record> = self.read()?;

        let record = records
            .into_iter()
            .find(|record| record.id == *id)
            .ok_or_else(|| eyre::eyre!("Workspace with id {} not found", id))?;

        let entity = record.load_entity();

        Ok(entity)
    }
}

impl list::List for json::Client {
    fn list(&self) -> Result<Vec<Entity>, Error> {
        let mut records: Vec<Record> = self.read()?;
        records.sort_unstable_by(|a, b| a.last_access_time.cmp(&b.last_access_time).reverse());

        let entities = records.into_iter().map(Record::load_entity).collect();

        Ok(entities)
    }
}

impl track_access_time::Track for json::Client {
    fn track(&self, entity: Entity) -> Result<Entity, Error> {
        let Some(id) = entity.id() else {
            return Err(
                eyre::eyre!("Attempt to track access time for workspace without id").into(),
            );
        };

        let mut records: Vec<Record> = self.read()?;
        let record = records
            .iter_mut()
            .find(|record| record.id == *id)
            .ok_or(eyre::eyre!("Workspace with id {} not found", id,))?;

        record.last_access_time = Some(chrono::Utc::now());

        self.save(records)?;

        use get::Get;
        self.get(id)
    }
}

impl update::Update for json::Client {
    fn update(&self, entity: Entity) -> Result<Entity, Error> {
        let Some(id) = entity.id() else {
            return Err(eyre::eyre!("Attemp to update workspace without id").into());
        };

        let mut records: Vec<Record> = self.read()?;
        let record = records
            .iter_mut()
            .find(|record| record.id == *id)
            .ok_or_else(|| eyre::eyre!("Workspace with id {}not found", id))?;

        record.name = entity.name().to_string();
        record.location = entity
            .location()
            .map(ToString::to_string)
            .unwrap_or_default();

        self.save(records)?;

        Ok(entity)
    }
}

impl Record {
    fn from_entity(entity: &Entity) -> Result<Self, eyre::Report> {
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
