use crate::{
    clients::memories,
    models::{EditWorkspaceModel, EditWorkspaceModelParameters},
    router::EditWorkspaceParameters,
    types::Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: EditWorkspaceParameters) -> Result<EditWorkspaceModel> {
        let EditWorkspaceParameters { id } = parameters;

        let workspace = self.memories.get_workspace(&id)?;

        let model = EditWorkspaceModel::new(EditWorkspaceModelParameters { workspace });

        Ok(model)
    }
}
