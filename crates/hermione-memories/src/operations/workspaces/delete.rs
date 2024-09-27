use crate::types::shared::{Id, Result};

pub trait Delete {
    fn delete(&self, workspace_id: Id) -> Result<()>;
}

pub struct Operation<'a, D> {
    pub deleter: &'a D,
}

impl<'a, D> Operation<'a, D>
where
    D: Delete,
{
    pub fn execute(&self, workspace_id: Id) -> Result<()> {
        self.deleter.delete(workspace_id)
    }
}
