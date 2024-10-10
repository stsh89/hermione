use crate::{
    coordinator::{workspaces::ListParameters, Coordinator},
    models::workspaces::list::{Model, ModelParameters},
    parameters::workspaces::{create::Parameters, list::PAGE_SIZE},
    presenters::workspace::Presenter,
    Result,
};

pub struct Handler<'a> {
    pub coordinator: &'a Coordinator,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters { name, location } = parameters;

        self.coordinator.workspaces().create(Presenter {
            id: String::new(),
            location: Some(location),
            name,
        })?;

        let workspaces = self.coordinator.workspaces().list(ListParameters {
            name_contains: "",
            page_number: 0,
            page_size: PAGE_SIZE,
        })?;

        let model = Model::new(ModelParameters {
            workspaces,
            search_query: String::new(),
            page_number: 0,
            page_size: PAGE_SIZE,
        })?;

        Ok(model)
    }
}
