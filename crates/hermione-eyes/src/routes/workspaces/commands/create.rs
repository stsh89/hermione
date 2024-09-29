use super::list::{Model, ModelParameters};
use crate::{
    clients::memories,
    router::workspaces::commands::CreateParameters,
    types::{Command, Result},
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: CreateParameters) -> Result<Model> {
        let CreateParameters {
            workspace_id,
            name,
            program,
        } = parameters;

        self.memories.create_command(Command {
            workspace_id: workspace_id.clone(),
            id: String::new(),
            name,
            program,
        })?;

        let workspace = self.memories.get_workspace(&workspace_id)?;
        let commands = self.memories.list_commands(&workspace_id)?;

        Model::new(ModelParameters {
            workspace,
            commands,
            search_query: String::new(),
        })
    }
}
