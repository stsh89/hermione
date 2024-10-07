use crate::{
    coordinator::Coordinator,
    models::workspaces::edit::{Model, ModelParameters},
    parameters::workspaces::edit::Parameters,
    Result,
};

pub struct Handler<'a> {
    pub coordinator: &'a Coordinator,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters { id } = parameters;

        let workspace = self.coordinator.workspaces().get(&id)?;

        let model = Model::new(ModelParameters { workspace });

        Ok(model)
    }
}
