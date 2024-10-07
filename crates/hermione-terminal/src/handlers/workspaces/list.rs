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
        let Parameters { search_query } = parameters;

        let name_contains = if search_query.is_empty() {
            None
        } else {
            Some(search_query.as_ref())
        };

        let workspaces = self
            .coordinator
            .workspaces()
            .list(ListParameters { name_contains })?;

        Model::new(ModelParameters {
            workspaces,
            search_query,
        })
    }
}
