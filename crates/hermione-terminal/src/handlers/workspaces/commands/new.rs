use crate::{
    coordinator::Coordinator,
    models::workspaces::commands::new::{Model, ModelParameters},
    parameters::workspaces::commands::new::Parameters,
    Result,
};

pub struct Handler<'a> {
    pub coordinator: &'a Coordinator,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters { workspace_id } = parameters;

        let workspace = self.coordinator.workspaces().get(&workspace_id)?;

        Model::new(ModelParameters { workspace })
    }
}
