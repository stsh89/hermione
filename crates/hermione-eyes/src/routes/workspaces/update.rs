use super::commands::list::{Model, ModelParameters};
use crate::{clients::memories, router::workspaces::UpdateParameters, Result};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: UpdateParameters) -> Result<Model> {
        let UpdateParameters { id, name, location } = parameters;

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
