use crate::{
    clients::organizer,
    models::{EditWorkspaceModel, EditWorkspaceModelParameters},
    router::EditWorkspaceParameters,
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a organizer::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: EditWorkspaceParameters) -> Result<EditWorkspaceModel> {
        let EditWorkspaceParameters { id } = parameters;

        let workspace = self.organizer.get_workspace(&id)?;

        let model = EditWorkspaceModel::new(EditWorkspaceModelParameters { workspace });

        Ok(model)
    }
}
