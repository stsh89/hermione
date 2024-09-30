mod json;

use crate::{dtos::command::Dto, Result};
use hermione_memories::{
    Id,
    {
        entities::command::ScopedId,
        operations::workspaces::commands::{
            create, delete, get, list, track_execution_time, update,
        },
    },
};
use std::{path::PathBuf, str::FromStr};

pub trait Operations {
    fn create(&self, data: Dto) -> Result<Dto>;
    fn delete(&self, workspace_id: &str, id: &str) -> Result<()>;
    fn get(&self, workspace_id: &str, id: &str) -> Result<Dto>;
    fn list(&self, workspace_id: &str) -> Result<Vec<Dto>>;
    fn track_execution_time(&self, workspace_id: &str, command_id: &str) -> Result<Dto>;
    fn update(&self, data: Dto) -> Result<Dto>;
}

pub struct Client {
    inner: json::Client,
}

impl Operations for Client {
    fn create(&self, data: Dto) -> anyhow::Result<Dto> {
        let workspace = create::Operation {
            creator: &self.inner,
        }
        .execute(data.new_entity()?)?;

        Ok(Dto::from_entity(workspace))
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

    fn get(&self, workspace_id: &str, id: &str) -> anyhow::Result<Dto> {
        let id = ScopedId {
            workspace_id: Id::from_str(workspace_id)?,
            id: Id::from_str(id)?,
        };

        let workspace = get::Operation {
            getter: &self.inner,
        }
        .execute(id)?;

        Ok(Dto::from_entity(workspace))
    }

    fn list(&self, workspace_id: &str) -> anyhow::Result<Vec<Dto>> {
        let workspaces = list::Operation {
            lister: &self.inner,
        }
        .execute(Id::from_str(workspace_id)?)?;

        Ok(workspaces.into_iter().map(Dto::from_entity).collect())
    }

    fn track_execution_time(&self, workspace_id: &str, id: &str) -> anyhow::Result<Dto> {
        let id = ScopedId {
            workspace_id: Id::from_str(workspace_id)?,
            id: Id::from_str(id)?,
        };

        use hermione_memories::operations::workspaces::commands::get::Get;
        let entity = self.inner.get(id)?;

        let entity = track_execution_time::Operation {
            tracker: &self.inner,
        }
        .execute(entity)?;

        Ok(Dto::from_entity(entity))
    }

    fn update(&self, data: Dto) -> anyhow::Result<Dto> {
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
