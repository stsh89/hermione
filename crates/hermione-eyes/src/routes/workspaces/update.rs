use crate::{
    clients::memories,
    models::workspaces::commands::list::{Model, ModelParameters},
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

pub struct Parameters {
    pub id: String,
    pub name: String,
    pub location: String,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters { id, name, location } = parameters;

        let mut workspace = self.memories.get_workspace(&id)?;

        workspace.name = name;
        workspace.location = Some(location);

        let workspace = self.memories.update_workspace(workspace)?;
        let commands = self.memories.list_commands(&id)?;

        let model = Model::new(ModelParameters {
            commands,
            workspace,
            search_query: String::new(),
        })?;

        Ok(model)
    }
}
