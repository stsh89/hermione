use crate::{
    clients::memories,
    router::CreateCommandParameters,
    routes::get_workspace::{Model, ModelParameters},
    types::{Command, Result},
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: CreateCommandParameters) -> Result<Model> {
        let CreateCommandParameters {
            workspace_id,
            name,
            program,
        } = parameters;

        self.memories.create_command(Command {
            workspace_id: workspace_id.clone(),
            id: None,
            name,
            program,
        })?;

        let workspace = self.memories.get_workspace(&workspace_id)?;
        let commands = self.memories.list_commands(&workspace_id)?;

        Model::new(ModelParameters {
            workspace,
            commands,
            commands_search_query: String::new(),
        })
    }
}
