use crate::models::NewCommandModel;

pub struct Handler {}

impl Handler {
    pub fn handle(self) -> NewCommandModel {
        NewCommandModel::new()
    }
}
