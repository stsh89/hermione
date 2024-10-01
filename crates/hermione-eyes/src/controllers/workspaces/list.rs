use crate::{
    clients::memories::Client,
    models::workspaces::list::{Model, ModelParameters},
    parameters::workspaces::list::Parameters,
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters { search_query } = parameters;
        let mut workspaces = self.memories.list_workspaces()?;
        let filter = search_query.to_lowercase();

        if !filter.is_empty() {
            workspaces.retain(|w| w.name.to_lowercase().contains(&filter));
        }

        Model::new(ModelParameters {
            workspaces,
            search_query,
        })
    }
}
