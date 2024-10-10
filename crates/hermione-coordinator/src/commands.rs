use crate::{connection, core, Result};
use hermione_core::{entities::command::Entity, operations::commands::list};
use std::path::Path;

pub trait Operations {
    fn list(&self, parameters: ListParameters) -> Result<Vec<Dto>>;
}

pub struct ListParameters {
    pub page_number: u32,
    pub page_size: u32,
}

pub struct Client {
    inner: core::commands::Client,
}

pub struct Dto {
    pub id: String,
    pub program: String,
    pub name: String,
    pub workspace_id: String,
}

impl Operations for Client {
    fn list(&self, parameters: ListParameters) -> Result<Vec<Dto>> {
        let ListParameters {
            page_number,
            page_size,
        } = parameters;

        let workspaces = list::Operation {
            lister: &self.inner,
        }
        .execute(list::Parameters {
            page_number,
            page_size,
        })?;

        Ok(workspaces.into_iter().map(Dto::from_entity).collect())
    }
}

impl Client {
    pub fn new(dir_path: &Path) -> Result<Self> {
        let connection = connection(dir_path)?;
        let inner = core::commands::Client::new(connection);

        Ok(Self { inner })
    }
}

impl Dto {
    fn from_entity(entity: Entity) -> Self {
        Self {
            id: entity.id().map(|id| id.to_string()).unwrap_or_default(),
            name: entity.name().to_string(),
            program: entity.program().to_string(),
            workspace_id: entity.workspace_id().to_string(),
        }
    }
}
