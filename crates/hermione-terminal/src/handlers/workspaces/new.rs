use crate::{models::workspaces::new::Model, Result};

pub struct Handler {}

impl Handler {
    pub fn handle(self) -> Result<Model> {
        Model::new()
    }
}
