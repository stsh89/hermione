use crate::{
    entities::Workspace,
    models::{EditWorkspaceModel, EditWorkspaceModelParameters},
};

pub struct Handler {
    pub workspace: Workspace,
}

impl Handler {
    pub fn handle(self) -> EditWorkspaceModel {
        EditWorkspaceModel::new(EditWorkspaceModelParameters {
            workspace: self.workspace,
        })
    }
}
