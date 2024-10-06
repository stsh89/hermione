use crate::{
    clients::memories,
    models::workspaces::commands::new::{Model, ModelParameters},
    parameters::workspaces::commands::new::Parameters,
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters { workspace_id } = parameters;

        let workspace = self.memories.get_workspace(&workspace_id)?;

        Ok(Model::new(ModelParameters { workspace }))
    }
}