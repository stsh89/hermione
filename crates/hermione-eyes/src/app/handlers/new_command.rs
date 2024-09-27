use crate::{
    clients::memories,
    models::{NewCommandModel, NewCommandModelParameters},
    router::NewCommandParameters,
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: NewCommandParameters) -> Result<NewCommandModel> {
        let NewCommandParameters { workspace_id } = parameters;

        let workspace = self.memories.get_workspace(&workspace_id)?;

        Ok(NewCommandModel::new(NewCommandModelParameters {
            workspace,
        }))
    }
}
