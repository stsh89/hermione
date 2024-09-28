use crate::{
    impls::json,
    types::workspace::{Data, Operations},
};
use hermione_memories::{
    operations::workspaces::{create, delete, get, list, track_access_time, update},
    types::{
        shared::{Error, Id},
        workspace::{Entity, LoadParameters, Location, Name},
    },
};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};

#[derive(Serialize, Deserialize)]
pub struct Record {
    id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    last_access_time: Option<chrono::DateTime<chrono::Utc>>,

    location: String,
    name: String,
}

pub struct Client {
    inner: json::Client,
}

impl create::Create for json::Client {
    fn create(&self, mut workspace: Entity) -> Result<Entity, Error> {
        let mut records: Vec<Record> = self.read()?;
        workspace.set_id(Id::generate())?;

        let record = Record::from_entity(&workspace)?;
        records.push(record);

        self.save(records)?;

        Ok(workspace)
    }
}

impl delete::Delete for json::Client {
    fn delete(&self, id: Id) -> Result<(), Error> {
        let mut records: Vec<Record> = self.read()?;
        let id = id.to_string();

        records.retain(|record| record.id != id);

        self.save(records)?;

        Ok(())
    }
}

impl get::Get for json::Client {
    fn get(&self, id: Id) -> Result<Entity, Error> {
        let records: Vec<Record> = self.read()?;
        let id = id.to_string();

        let record = records
            .into_iter()
            .find(|record| record.id == id)
            .ok_or_else(|| eyre::eyre!("Workspace with id {} not found", id))?;

        let entity = record.load_entity()?;

        Ok(entity)
    }
}

impl list::List for json::Client {
    fn list(&self) -> Result<Vec<Entity>, Error> {
        let records: Vec<Record> = self.read()?;

        let entities = records
            .into_iter()
            .map(|record| record.load_entity())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entities)
    }
}

impl track_access_time::Track for json::Client {
    fn track(&self, workspace: Entity) -> Result<Entity, Error> {
        let Some(id) = workspace.id().map(|id| id.to_string()) else {
            return Err(
                eyre::eyre!("Attempt to track access time for workspace without id").into(),
            );
        };

        let mut records: Vec<Record> = self.read()?;
        let record = records
            .iter_mut()
            .find(|record| record.id == id)
            .ok_or(eyre::eyre!("Workspace with id {} not found", id,))?;

        record.last_access_time = Some(chrono::Utc::now());
        let entity = record.load_entity()?;

        self.save(records)?;

        Ok(entity)
    }
}

impl update::Update for json::Client {
    fn update(&self, workspace: Entity) -> Result<Entity, Error> {
        let Some(id) = workspace.id().map(|id| id.to_string()) else {
            return Err(eyre::eyre!("Attemp to update workspace without id").into());
        };

        let mut records: Vec<Record> = self.read()?;
        let record = records
            .iter_mut()
            .find(|record| record.id == id)
            .ok_or_else(|| eyre::eyre!("Workspace with id {}not found", id))?;

        record.name = workspace.name().to_string();
        record.location = workspace
            .location()
            .map(ToString::to_string)
            .unwrap_or_default();

        self.save(records)?;

        Ok(workspace)
    }
}

impl Operations for Client {
    fn create(&self, data: Data) -> anyhow::Result<Data> {
        let workspace = create::Operation {
            creator: &self.inner,
        }
        .execute(data.new_entity())?;

        Ok(Data::from_entity(workspace))
    }

    fn delete(&self, id: &str) -> anyhow::Result<()> {
        delete::Operation {
            deleter: &self.inner,
        }
        .execute(Id::from_str(id)?)?;

        Ok(())
    }

    fn get(&self, id: &str) -> anyhow::Result<Data> {
        let workspace = get::Operation {
            getter: &self.inner,
        }
        .execute(Id::from_str(id)?)?;

        Ok(Data::from_entity(workspace))
    }

    fn list(&self) -> anyhow::Result<Vec<Data>> {
        let workspaces = list::Operation {
            lister: &self.inner,
        }
        .execute()?;

        Ok(workspaces.into_iter().map(Data::from_entity).collect())
    }

    fn track_access_time(&self, id: &str) -> anyhow::Result<Data> {
        use hermione_memories::operations::workspaces::get::Get;

        let entity = self.inner.get(Id::from_str(id)?)?;

        let entity = track_access_time::Operation {
            tracker: &self.inner,
        }
        .execute(entity)?;

        Ok(Data::from_entity(entity))
    }

    fn update(&self, data: Data) -> anyhow::Result<Data> {
        let workspace = update::Operation {
            updater: &self.inner,
        }
        .execute(data.load_entity()?)?;

        Ok(Data::from_entity(workspace))
    }
}

impl Client {
    pub fn new(path: PathBuf) -> Self {
        Self {
            inner: json::Client::new(path),
        }
    }
}

impl Record {
    fn from_entity(entity: &Entity) -> Result<Self, eyre::Report> {
        Ok(Self {
            id: entity
                .id()
                .map(|id| id.to_string())
                .ok_or(eyre::eyre!("Record without id"))?,
            last_access_time: entity.last_access_time().map(From::from),
            location: entity
                .location()
                .map(ToString::to_string)
                .unwrap_or_default(),
            name: entity.name().to_string(),
        })
    }

    fn load_entity(&self) -> Result<Entity, eyre::Report> {
        Ok(Entity::load(LoadParameters {
            id: Id::from_str(&self.id)?,
            last_access_time: self.last_access_time.map(From::from),
            location: Some(Location::new(self.location.clone())),
            name: Name::new(self.name.clone()),
        }))
    }
}
