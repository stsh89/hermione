use crate::{
    app::UpdateWorkspaceParameters,
    clients::memories,
    routes::workspaces::get::{Model, ModelParameters},
    types::Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: UpdateWorkspaceParameters) -> Result<Model> {
        let UpdateWorkspaceParameters { id, name, location } = parameters;

        let mut workspace = self.memories.get_workspace(&id)?;

        workspace.name = name;
        workspace.location = location;

        let workspace = self.memories.update_workspace(workspace)?;
        let commands = self.memories.list_commands(&id)?;

        let model = Model::new(ModelParameters {
            commands,
            workspace,
            commands_search_query: String::new(),
        })?;

        Ok(model)
    }
}
