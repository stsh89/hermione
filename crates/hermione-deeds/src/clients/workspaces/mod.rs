use crate::types::workspace::{Data, Operations};
use hermione_memories::{
    operations::workspaces::{create, delete, get, list, track_access_time, update},
    Id,
};
use std::str::FromStr;

pub mod commands;
pub mod json;

pub struct Client<T>
where
    T: get::Get
        + list::List
        + create::Create
        + delete::Delete
        + update::Update
        + track_access_time::Track,
{
    inner: T,
}

impl<T> Operations for Client<T>
where
    T: get::Get
        + list::List
        + create::Create
        + delete::Delete
        + update::Update
        + track_access_time::Track,
{
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

impl<T> Client<T>
where
    T: get::Get
        + list::List
        + create::Create
        + delete::Delete
        + update::Update
        + track_access_time::Track,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}
