use crate::models::NewWorkspaceModel;

pub struct Handler {}

impl Handler {
    pub fn handle(self) -> NewWorkspaceModel {
        NewWorkspaceModel::new()
    }
}
