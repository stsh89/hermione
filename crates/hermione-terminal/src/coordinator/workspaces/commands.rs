use crate::{presenters::command::Presenter, Result};
use hermione_coordinator::workspaces::commands::{self, Client, Operations};
use std::path::Path;

pub struct Coordinator {
    client: Client,
}

pub struct ListParameters<'a> {
    pub page_number: u32,
    pub page_size: u32,
    pub program_contains: &'a str,
    pub workspace_id: &'a str,
}

impl Coordinator {
    pub fn new(app_path: &Path) -> Result<Self> {
        Ok(Self {
            client: Client::new(app_path)?,
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
            program_contains,
            page_number,
            page_size,
        } = parameters;

        let commands = self.client.list(commands::ListParameters {
            workspace_id,
            program_contains,
            page_number,
            page_size,
        })?;

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
