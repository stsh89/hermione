use crate::types::{command::Entity, shared::Result};

pub trait Update {
    fn update(&self, command: Entity) -> Result<Entity>;
}

pub struct Operation<'a, U> {
    pub updater: &'a U,
}

impl<'a, U> Operation<'a, U>
where
    U: Update,
{
    pub fn execute(&self, command: Entity) -> Result<Entity> {
        self.updater.update(command)
    }
}
