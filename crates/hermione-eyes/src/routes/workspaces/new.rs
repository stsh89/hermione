use crate::models::workspaces::new::Model;

pub struct Handler {}

impl Handler {
    pub fn handle(self) -> Model {
        Model::new()
    }
}
