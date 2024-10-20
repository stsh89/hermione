pub mod commands;

use crate::{presenters::workspace::Presenter, Result};
use hermione_coordinator::{
    workspaces::{self, Client, Operations},
    Connection,
};
use std::rc::Rc;

pub struct Coordinator {
    client: Client,
    commands: commands::Coordinator,
}

pub struct ListParameters<'a> {
    pub name_contains: &'a str,
    pub page_number: u32,
    pub page_size: u32,
}

impl Coordinator {
    pub fn new(connection: Rc<Connection>) -> Self {
        Self {
            client: Client::new(connection.clone()),
            commands: commands::Coordinator::new(connection),
        }
    }

    pub fn create(&self, workspace: Presenter) -> Result<Presenter> {
        let data = self.client.create(workspace.into())?;

        Ok(data.into())
    }

    pub fn commands(&self) -> &commands::Coordinator {
        &self.commands
    }

    pub fn delete(&self, workspace_id: &str) -> Result<()> {
        self.client.delete(workspace_id)?;

        Ok(())
    }

    pub fn get(&self, workspace_id: &str) -> Result<Presenter> {
        let workspace = self.client.get(workspace_id)?;

        Ok(workspace.into())
    }

    pub fn list(&self, parameters: ListParameters) -> Result<Vec<Presenter>> {
        let ListParameters {
            name_contains,
            page_number,
            page_size,
        } = parameters;

        let workspaces = self.client.list(workspaces::ListParameters {
            name_contains,
            page_number,
            page_size,
        })?;

        Ok(workspaces.into_iter().map(Into::into).collect())
    }

    pub fn track_access_time(&self, workspace: Presenter) -> Result<Presenter> {
        let workspace = self.client.track_access_time(&workspace.id)?;

        Ok(workspace.into())
    }

    pub fn update(&self, workspace: Presenter) -> Result<Presenter> {
        let workspace = self.client.update(workspace.into())?;

        Ok(workspace.into())
    }
}
