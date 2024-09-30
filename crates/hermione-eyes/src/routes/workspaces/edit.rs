use crate::{
    app::router::workspaces::EditParameters,
    clients::memories::Client,
    models::workspaces::edit::{Model, ModelParameters},
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: EditParameters) -> Result<Model> {
        let EditParameters { id } = parameters;

        let workspace = self.memories.get_workspace(&id)?;

        let model = Model::new(ModelParameters { workspace });

        Ok(model)
    }
}
