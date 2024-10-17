use crate::{entities::workspace::Entity, Id, Result};

pub trait Find {
    fn find(&self, workspace_id: Id) -> Result<Option<Entity>>;
}

pub struct Operation<'a, R> {
    pub finder: &'a R,
}

impl<'a, R> Operation<'a, R>
where
    R: Find,
{
    pub fn execute(&self, workspace_id: Id) -> Result<Option<Entity>> {
        self.finder.find(workspace_id)
    }
}
