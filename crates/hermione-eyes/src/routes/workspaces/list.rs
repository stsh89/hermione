use crate::{
    app::router::workspaces::ListParameters,
    clients::memories::Client,
    models::workspaces::list::{Model, ModelParameters},
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: ListParameters) -> Result<Model> {
        let ListParameters { search_query } = parameters;
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
