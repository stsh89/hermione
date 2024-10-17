use crate::{entities::command::Entity, Result};

pub trait Import {
    fn import(&self, command: Entity) -> Result<Entity>;
}

pub struct Operation<'a, S> {
    pub importer: &'a S,
}

impl<'a, S> Operation<'a, S>
where
    S: Import,
{
    pub fn execute(&self, command: Entity) -> Result<Entity> {
        self.importer.import(command)
    }
}
