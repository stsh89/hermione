use anyhow::Result;
use chrono::{DateTime, Utc};
use hermione_memories::{
    entities::command::{Entity, LoadParameters, Name, NewParameters, Program},
    Id,
};
use std::str::FromStr;

pub trait WorkspaceOperations {
    fn create(&self, data: Data) -> Result<Data>;
    fn delete(&self, workspace_id: &str, id: &str) -> Result<()>;
    fn get(&self, workspace_id: &str, id: &str) -> Result<Data>;
    fn list(&self, workspace_id: &str) -> Result<Vec<Data>>;
    fn track_execution_time(&self, workspace_id: &str, command_id: &str) -> Result<Data>;
    fn update(&self, data: Data) -> Result<Data>;
}

pub struct Data {
    pub id: String,
    pub last_execute_time: Option<DateTime<Utc>>,
    pub name: String,
    pub program: String,
    pub workspace_id: String,
}

impl Data {
    pub(crate) fn from_entity(entity: Entity) -> Self {
        Self {
            id: entity.id().map(|id| id.to_string()).unwrap_or_default(),
            name: entity.name().to_string(),
            last_execute_time: entity.last_execute_time().map(Into::into),
            program: entity.program().to_string(),
            workspace_id: entity.workspace_id().to_string(),
        }
    }

    pub(crate) fn load_entity(self) -> Result<Entity> {
        Ok(Entity::load(LoadParameters {
            id: Id::from_str(&self.id)?,
            name: Name::new(self.name),
            last_execute_time: self.last_execute_time.map(From::from),
            program: Program::new(self.program),
            workspace_id: Id::from_str(&self.workspace_id)?,
        }))
    }

    pub(crate) fn new_entity(self) -> Result<Entity> {
        Ok(Entity::new(NewParameters {
            name: Name::new(self.name),
            program: Program::new(self.program),
            workspace_id: Id::from_str(&self.workspace_id)?,
        }))
    }
}
