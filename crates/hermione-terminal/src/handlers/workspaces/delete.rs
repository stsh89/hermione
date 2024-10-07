use crate::{
    coordinator::{workspaces::ListParameters, Coordinator},
    models::workspaces::list::{Model, ModelParameters},
    parameters::workspaces::delete::Parameters,
    Result,
};

pub struct Handler<'a> {
    pub coordinator: &'a Coordinator,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters { id } = parameters;

        self.coordinator.workspaces().delete(&id)?;
        let workspaces = self
            .coordinator
            .workspaces()
            .list(ListParameters::default())?;

        let model = Model::new(ModelParameters {
            workspaces,
            search_query: String::new(),
        })?;

        Ok(model)
    }
}
