use crate::{
    app::router::workspaces::commands::NewParameters,
    clients::memories,
    models::workspaces::commands::new::{Model, ModelParameters},
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: NewParameters) -> Result<Model> {
        let NewParameters { workspace_id } = parameters;

        let workspace = self.memories.get_workspace(&workspace_id)?;

        Ok(Model::new(ModelParameters { workspace }))
    }
}
