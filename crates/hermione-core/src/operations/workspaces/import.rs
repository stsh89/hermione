use crate::{entities::workspace::Entity, Result};

pub trait Import {
    fn import(&self, workspace: Entity) -> Result<Entity>;
}

pub struct Operation<'a, S> {
    pub importer: &'a S,
}

impl<'a, S> Operation<'a, S>
where
    S: Import,
{
    pub fn execute(&self, workspace: Entity) -> Result<Entity> {
        self.importer.import(workspace)
    }
}
