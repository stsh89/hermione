pub mod commands;

use crate::{presenters::workspace::Presenter, Result};
use hermione_coordinator::workspaces::{self, Client, Operations};
use std::path::Path;

pub struct Coordinator {
    client: Client,
    commands: commands::Coordinator,
}

#[derive(Default)]
pub struct ListParameters<'a> {
    pub name_contains: Option<&'a str>,
}

impl Coordinator {
    pub fn new(app_path: &Path) -> Result<Self> {
        Ok(Self {
            client: Client::new(app_path)?,
            commands: commands::Coordinator::new(app_path)?,
        })
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
        let ListParameters { name_contains } = parameters;

        let workspaces = self
            .client
            .list(workspaces::ListParameters { name_contains })?;

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
