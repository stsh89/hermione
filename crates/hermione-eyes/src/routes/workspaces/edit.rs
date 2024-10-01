use crate::{
    clients::memories::Client,
    models::workspaces::edit::{Model, ModelParameters},
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a Client,
}

pub struct Parameters {
    pub id: String,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters { id } = parameters;

        let workspace = self.memories.get_workspace(&id)?;

        let model = Model::new(ModelParameters { workspace });

        Ok(model)
    }
}
