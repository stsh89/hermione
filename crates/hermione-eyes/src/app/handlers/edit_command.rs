use crate::{
    entities::Command,
    models::{EditCommandModel, EditCommandModelParameters},
};

pub struct Handler {
    pub command: Command,
}

impl Handler {
    pub fn handle(self) -> EditCommandModel {
        EditCommandModel::new(EditCommandModelParameters {
            command: self.command,
        })
    }
}
