use crate::{
    coordinator::{workspaces::ListParameters, Coordinator},
    models::workspaces::list::{Model, ModelParameters},
    parameters::workspaces::list::Parameters,
    Result,
};

pub struct Handler<'a> {
    pub coordinator: &'a Coordinator,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters {
            search_query,
            page_number,
            page_size,
        } = parameters;

        let workspaces = self.coordinator.workspaces().list(ListParameters {
            name_contains: &search_query,
            page_number,
            page_size,
        })?;

        Model::new(ModelParameters {
            workspaces,
            search_query,
            page_number,
            page_size,
        })
    }
}
