use crate::{
    clients::organizer,
    models::{NewCommandModel, NewCommandModelParameters},
    router::NewCommandParameters,
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a organizer::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: NewCommandParameters) -> Result<NewCommandModel> {
        let NewCommandParameters { workspace_id } = parameters;

        let workspace = self.organizer.get_workspace(&workspace_id)?;

        Ok(NewCommandModel::new(NewCommandModelParameters {
            workspace,
        }))
    }
}
