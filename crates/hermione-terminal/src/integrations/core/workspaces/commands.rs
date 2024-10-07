use crate::{presenters, Result};
use anyhow::anyhow;
use hermione_coordinator::workspaces;
use std::path::Path;

pub struct Client {
    inner: Box<dyn workspaces::commands::Operations>,
}

pub struct ListParameters<'a> {
    pub workspace_id: &'a str,
    pub search_query: Option<&'a str>,
}

impl Client {
    pub fn json(app_path: &Path) -> Result<Self> {
        let inner = Box::new(hermione_coordinator::workspaces::commands::Client::new(
            hermione_coordinator::json::workspaces::commands::Client::new(
                app_path.join("commands.json"),
            )
            .map_err(|err| anyhow!(err))?,
        ));

        Ok(Self { inner })
    }

    pub fn create(
        &self,
        command: presenters::command::Presenter,
    ) -> Result<presenters::command::Presenter> {
        let data = self.inner.create(command.into())?;

        Ok(data.into())
    }

    pub fn delete(&self, workspace_id: &str, command_id: &str) -> Result<()> {
        self.inner.delete(workspace_id, command_id)?;

        Ok(())
    }

    pub fn get(
        &self,
        workspace_id: &str,
        command_id: &str,
    ) -> Result<presenters::command::Presenter> {
        let command = self.inner.get(workspace_id, command_id)?;

        Ok(command.into())
    }

    pub fn list(&self, parameters: ListParameters) -> Result<Vec<presenters::command::Presenter>> {
        let ListParameters {
            workspace_id,
            search_query,
        } = parameters;

        let commands = self.inner.list(workspace_id)?;

        let filter = search_query.as_ref().map(|query| query.to_lowercase());

        let commands = if let Some(filter) = filter {
            commands
                .into_iter()
                .filter(|c| c.program.to_lowercase().contains(&filter))
                .collect()
        } else {
            commands
        };

        Ok(commands.into_iter().map(Into::into).collect())
    }

    pub fn track_execution_time(
        &self,
        command: presenters::command::Presenter,
    ) -> Result<presenters::command::Presenter> {
        let command = self
            .inner
            .track_execution_time(&command.workspace_id, &command.id)?;

        Ok(command.into())
    }

    pub fn update(
        &self,
        command: presenters::command::Presenter,
    ) -> Result<presenters::command::Presenter> {
        let command = self.inner.update(command.into())?;

        Ok(command.into())
    }
}
