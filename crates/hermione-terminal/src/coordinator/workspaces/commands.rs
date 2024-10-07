use crate::{presenters::command::Presenter, Result};
use hermione_coordinator::workspaces::commands::{Client, Operations};
use std::path::Path;

pub struct Coordinator {
    client: Client,
}

pub struct ListParameters<'a> {
    pub workspace_id: &'a str,
    pub search_query: Option<&'a str>,
}

impl Coordinator {
    pub fn new(app_path: &Path) -> Result<Self> {
        let commands_path = app_path.join("commands.json");

        Ok(Self {
            client: Client::new(commands_path)?,
        })
    }

    pub fn create(&self, command: Presenter) -> Result<Presenter> {
        let data = self.client.create(command.into())?;

        Ok(data.into())
    }

    pub fn delete(&self, workspace_id: &str, command_id: &str) -> Result<()> {
        self.client.delete(workspace_id, command_id)?;

        Ok(())
    }

    pub fn get(&self, workspace_id: &str, command_id: &str) -> Result<Presenter> {
        let command = self.client.get(workspace_id, command_id)?;

        Ok(command.into())
    }

    pub fn list(&self, parameters: ListParameters) -> Result<Vec<Presenter>> {
        let ListParameters {
            workspace_id,
            search_query,
        } = parameters;

        let commands = self.client.list(workspace_id)?;

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

    pub fn track_execution_time(&self, command: Presenter) -> Result<Presenter> {
        let command = self
            .client
            .track_execution_time(&command.workspace_id, &command.id)?;

        Ok(command.into())
    }

    pub fn update(&self, command: Presenter) -> Result<Presenter> {
        let command = self.client.update(command.into())?;

        Ok(command.into())
    }
}
