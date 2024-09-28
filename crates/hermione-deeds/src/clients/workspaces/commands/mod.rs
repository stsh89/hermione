use std::str::FromStr;

use crate::types::command::{Data, WorkspaceOperations};
use hermione_memories::{
    operations::workspaces::commands::{create, delete, get, list, track_execution_time, update},
    types::{command::ScopedId, Id},
};

pub mod json;

pub struct Client<T>
where
    T: get::Get
        + list::List
        + create::Create
        + delete::Delete
        + update::Update
        + track_execution_time::Track,
{
    inner: T,
}

impl<T> WorkspaceOperations for Client<T>
where
    T: get::Get
        + list::List
        + create::Create
        + delete::Delete
        + update::Update
        + track_execution_time::Track,
{
    fn create(&self, data: Data) -> anyhow::Result<Data> {
        let workspace = create::Operation {
            creator: &self.inner,
        }
        .execute(data.new_entity()?)?;

        Ok(Data::from_entity(workspace))
    }

    fn delete(&self, workspace_id: &str, id: &str) -> anyhow::Result<()> {
        let id = ScopedId {
            workspace_id: Id::from_str(workspace_id)?,
            id: Id::from_str(id)?,
        };

        delete::Operation {
            deleter: &self.inner,
        }
        .execute(id)?;

        Ok(())
    }

    fn get(&self, workspace_id: &str, id: &str) -> anyhow::Result<Data> {
        let id = ScopedId {
            workspace_id: Id::from_str(workspace_id)?,
            id: Id::from_str(id)?,
        };

        let workspace = get::Operation {
            getter: &self.inner,
        }
        .execute(id)?;

        Ok(Data::from_entity(workspace))
    }

    fn list(&self, workspace_id: &str) -> anyhow::Result<Vec<Data>> {
        let workspaces = list::Operation {
            lister: &self.inner,
        }
        .execute(Id::from_str(workspace_id)?)?;

        Ok(workspaces.into_iter().map(Data::from_entity).collect())
    }

    fn track_execution_time(&self, workspace_id: &str, id: &str) -> anyhow::Result<Data> {
        let id = ScopedId {
            workspace_id: Id::from_str(workspace_id)?,
            id: Id::from_str(id)?,
        };

        let entity = self.inner.get(id)?;

        let entity = track_execution_time::Operation {
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
        + track_execution_time::Track,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}
