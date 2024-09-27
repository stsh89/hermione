use crate::types::{shared::Result, workspace::Entity};

pub trait Update {
    fn update(&self, workspace: Entity) -> Result<Entity>;
}

pub struct Operation<'a, U> {
    pub updater: &'a U,
}

impl<'a, U> Operation<'a, U>
where
    U: Update,
{
    pub fn execute(&self, workspace: Entity) -> Result<Entity> {
        self.updater.update(workspace)
    }
}