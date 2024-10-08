use crate::{entities::workspace::Entity, Id, Result};

pub trait Get {
    fn get(&self, workspace_id: Id) -> Result<Entity>;
}

pub struct Operation<'a, R> {
    pub getter: &'a R,
}

impl<'a, R> Operation<'a, R>
where
    R: Get,
{
    pub fn execute(&self, workspace_id: Id) -> Result<Entity> {
        self.getter.get(workspace_id)
    }
}
