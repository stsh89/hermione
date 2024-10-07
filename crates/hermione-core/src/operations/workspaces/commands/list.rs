use crate::{entities::command::Entity, Id, Result};

pub trait List {
    fn list(&self, workspace_id: Id) -> Result<Vec<Entity>>;
}

pub struct Operation<'a, L>
where
    L: List,
{
    pub lister: &'a L,
}

impl<'a, L> Operation<'a, L>
where
    L: List,
{
    pub fn execute(&self, workspace_id: Id) -> Result<Vec<Entity>> {
        self.lister.list(workspace_id)
    }
}
