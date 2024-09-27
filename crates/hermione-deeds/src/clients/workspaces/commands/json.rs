use super::Operations;
use crate::{clients::shared::json, types::command::Data};
use hermione_memories::{
    operations::workspaces::commands::{create, delete, get, list, track_execution_time, update},
    types::{
        command::{Entity, LoadParameters, Name, Program},
        shared::{Error, Id, ScopedId},
    },
};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};

#[derive(Serialize, Deserialize)]
pub struct Record {
    id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execute_time: Option<chrono::DateTime<chrono::Utc>>,

    name: String,
    program: String,
    workspace_id: String,
}

pub struct Client {
    inner: json::Client,
}

impl create::Create for json::Client {
    fn create(&self, mut entity: Entity) -> Result<Entity, Error> {
        let mut records: Vec<Record> = self.read()?;
        entity.set_id(Id::generate())?;

        let record = Record::from_entity(&entity)?;
        records.push(record);

        self.save(records)?;

        Ok(entity)
    }
}

impl delete::Delete for json::Client {
    fn delete(&self, id: ScopedId) -> Result<(), Error> {
        let ScopedId(workspace_id, id) = id;
        let mut records: Vec<Record> = self.read()?;

        let workspace_id = workspace_id.to_string();
        let id = id.to_string();

        records.retain(|record| !(record.id == id && record.workspace_id == workspace_id));

        self.save(records)?;

        Ok(())
    }
}

impl get::Get for json::Client {
    fn get(&self, id: ScopedId) -> Result<Entity, Error> {
        let ScopedId(workspace_id, id) = id;
        let records: Vec<Record> = self.read()?;

        let workspace_id = workspace_id.to_string();
        let id = id.to_string();

        let record = records
            .into_iter()
            .find(|record| record.id == id && record.workspace_id == workspace_id)
            .ok_or_else(|| eyre::eyre!("Workspace with id {} not found", id))?;

        let entity = record.load_entity()?;

        Ok(entity)
    }
}

impl list::List for json::Client {
    fn list(&self, workspace_id: Id) -> Result<Vec<Entity>, Error> {
        let records: Vec<Record> = self.read()?;
        let id = workspace_id.to_string();

        let entities = records
            .into_iter()
            .filter(|record| record.workspace_id == id)
            .map(|record| record.load_entity())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entities)
    }
}

impl track_execution_time::Track for json::Client {
    fn track(&self, entity: Entity) -> Result<Entity, Error> {
        let Some(id) = entity.get_id().map(|id| id.to_string()) else {
            return Err(
                eyre::eyre!("Attempt to track access time for workspace without id").into(),
            );
        };

        let mut records: Vec<Record> = self.read()?;
        let workspace_id = entity.workspace_id().to_string();

        let record = records
            .iter_mut()
            .find(|record| record.id == id && record.workspace_id == workspace_id)
            .ok_or(eyre::eyre!("Workspace with id {} not found", id,))?;

        record.last_execute_time = Some(chrono::Utc::now());
        let entity = record.load_entity()?;

        self.save(records)?;

        Ok(entity)
    }
}

impl update::Update for json::Client {
    fn update(&self, entity: Entity) -> Result<Entity, Error> {
        let Some(id) = entity.get_id().map(|id| id.to_string()) else {
            return Err(eyre::eyre!("Attemp to update workspace without id").into());
        };

        let mut records: Vec<Record> = self.read()?;
        let workspace_id = entity.workspace_id().to_string();

        let record = records
            .iter_mut()
            .find(|record| record.id == id && record.workspace_id == workspace_id)
            .ok_or_else(|| eyre::eyre!("Workspace with id {}not found", id))?;

        record.name = entity.name().to_string();
        record.program = entity.program().to_string();

        self.save(records)?;

        Ok(entity)
    }
}

impl Operations for Client {
    fn create(&self, data: Data) -> Result<Data, eyre::Report> {
        let workspace = create::Operation {
            creator: &self.inner,
        }
        .execute(data.new_entity()?)?;

        Ok(Data::from_entity(workspace))
    }

    fn delete(&self, workspace_id: &str, id: &str) -> Result<(), eyre::Report> {
        delete::Operation {
            deleter: &self.inner,
        }
        .execute(ScopedId(Id::from_str(workspace_id)?, Id::from_str(id)?))?;

        Ok(())
    }

    fn get(&self, workspace_id: &str, id: &str) -> Result<Data, eyre::Report> {
        let workspace = get::Operation {
            getter: &self.inner,
        }
        .execute(ScopedId(Id::from_str(workspace_id)?, Id::from_str(id)?))?;

        Ok(Data::from_entity(workspace))
    }

    fn list(&self, workspace_id: &str) -> Result<Vec<Data>, eyre::Report> {
        let workspaces = list::Operation {
            lister: &self.inner,
        }
        .execute(Id::from_str(workspace_id)?)?;

        Ok(workspaces.into_iter().map(Data::from_entity).collect())
    }

    fn track_execution_time(&self, workspace_id: &str, id: &str) -> Result<Data, eyre::Report> {
        use hermione_memories::operations::workspaces::commands::get::Get;

        let entity = self
            .inner
            .get(ScopedId(Id::from_str(workspace_id)?, Id::from_str(id)?))?;

        let entity = track_execution_time::Operation {
            tracker: &self.inner,
        }
        .execute(entity)?;

        Ok(Data::from_entity(entity))
    }

    fn update(&self, data: Data) -> Result<Data, eyre::Report> {
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
                .get_id()
                .map(|id| id.to_string())
                .ok_or(eyre::eyre!("Record without id"))?,
            last_execute_time: entity.last_execute_time().map(From::from),
            program: entity.program().to_string(),
            name: entity.name().to_string(),
            workspace_id: entity.workspace_id().to_string(),
        })
    }

    fn load_entity(&self) -> Result<Entity, eyre::Report> {
        Ok(Entity::load(LoadParameters {
            id: Id::from_str(&self.id)?,
            last_execute_time: self.last_execute_time.map(From::from),
            program: Program::new(self.program.clone()),
            name: Name::new(self.name.clone()),
            workspace_id: Id::from_str(&self.workspace_id)?,
        }))
    }
}
