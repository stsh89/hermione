pub mod commands;

use crate::{presenters, Result};
use anyhow::anyhow;
use hermione_coordinator;
use std::path::Path;

pub struct Client {
    workspaces: Box<dyn hermione_coordinator::workspaces::Operations>,
    commands: commands::Client,
}

impl Client {
    pub fn json(app_path: &Path) -> Result<Self> {
        let workspaces = Box::new(hermione_coordinator::workspaces::Client::new(
            hermione_coordinator::json::workspaces::Client::new(app_path.join("workspaces.json"))
                .map_err(|err| anyhow!(err))?,
        ));

        let commands = commands::Client::json(app_path)?;

        Ok(Self {
            workspaces,
            commands,
        })
    }

    pub fn create(
        &self,
        workspace: presenters::workspace::Presenter,
    ) -> Result<presenters::workspace::Presenter> {
        let data = self.workspaces.create(workspace.into())?;

        Ok(data.into())
    }

    pub fn commands(&self) -> &commands::Client {
        &self.commands
    }

    pub fn delete(&self, workspace_id: &str) -> Result<()> {
        self.workspaces.delete(workspace_id)?;

        Ok(())
    }

    pub fn get(&self, workspace_id: &str) -> Result<presenters::workspace::Presenter> {
        let workspace = self.workspaces.get(workspace_id)?;

        Ok(workspace.into())
    }

    pub fn list(&self) -> Result<Vec<presenters::workspace::Presenter>> {
        let workspaces = self.workspaces.list()?;

        Ok(workspaces.into_iter().map(Into::into).collect())
    }

    pub fn track_access_time(
        &self,
        workspace: presenters::workspace::Presenter,
    ) -> Result<presenters::workspace::Presenter> {
        let workspace = self.workspaces.track_access_time(&workspace.id)?;

        Ok(workspace.into())
    }

    pub fn update(
        &self,
        workspace: presenters::workspace::Presenter,
    ) -> Result<presenters::workspace::Presenter> {
        let workspace = self.workspaces.update(workspace.into())?;

        Ok(workspace.into())
    }
}
