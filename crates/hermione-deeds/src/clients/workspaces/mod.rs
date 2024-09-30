mod json;

pub mod commands;

use crate::{dtos::workspace::Dto, Result};
use hermione_memories::{
    operations::workspaces::{create, delete, get, list, track_access_time, update},
    Id,
};
use std::{path::PathBuf, str::FromStr};

pub trait Operations {
    fn create(&self, data: Dto) -> Result<Dto>;
    fn delete(&self, id: &str) -> Result<()>;
    fn get(&self, id: &str) -> Result<Dto>;
    fn list(&self) -> Result<Vec<Dto>>;
    fn track_access_time(&self, id: &str) -> Result<Dto>;
    fn update(&self, data: Dto) -> Result<Dto>;
}

pub struct Client {
    inner: json::Client,
}

impl Operations for Client {
    fn create(&self, data: Dto) -> Result<Dto> {
        let workspace = create::Operation {
            creator: &self.inner,
        }
        .execute(data.new_entity())?;

        Ok(Dto::from_entity(workspace))
    }

    fn delete(&self, id: &str) -> Result<()> {
        delete::Operation {
            deleter: &self.inner,
        }
        .execute(Id::from_str(id)?)?;

        Ok(())
    }

    fn get(&self, id: &str) -> Result<Dto> {
        let workspace = get::Operation {
            getter: &self.inner,
        }
        .execute(Id::from_str(id)?)?;

        Ok(Dto::from_entity(workspace))
    }

    fn list(&self) -> Result<Vec<Dto>> {
        let workspaces = list::Operation {
            lister: &self.inner,
        }
        .execute()?;

        Ok(workspaces.into_iter().map(Dto::from_entity).collect())
    }

    fn track_access_time(&self, id: &str) -> Result<Dto> {
        use hermione_memories::operations::workspaces::get::Get;
        let entity = self.inner.get(Id::from_str(id)?)?;

        let entity = track_access_time::Operation {
            tracker: &self.inner,
        }
        .execute(entity)?;

        Ok(Dto::from_entity(entity))
    }

    fn update(&self, data: Dto) -> Result<Dto> {
        let workspace = update::Operation {
            updater: &self.inner,
        }
        .execute(data.load_entity()?)?;

        Ok(Dto::from_entity(workspace))
    }
}

impl Client {
    pub fn new(path: PathBuf) -> Self {
        Self {
            inner: json::Client { path },
        }
    }
}
