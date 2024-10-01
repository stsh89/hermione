use crate::{
    clients::memories,
    models::workspaces::commands::list::{Model, ModelParameters},
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

pub struct Parameters {
    pub command_id: String,
    pub workspace_id: String,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters {
            workspace_id,
            command_id,
        } = parameters;

        self.memories.delete_command(&workspace_id, &command_id)?;

        let workspace = self.memories.get_workspace(&workspace_id)?;
        let commands = self.memories.list_commands(&workspace_id)?;

        Model::new(ModelParameters {
            workspace,
            search_query: String::new(),
            commands,
        })
    }
}
