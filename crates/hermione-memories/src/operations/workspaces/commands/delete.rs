use crate::types::{command::WorkspaceScopeId, shared::Result};

pub trait Delete {
    fn delete(&self, id: WorkspaceScopeId) -> Result<()>;
}

pub struct Operation<'a, D> {
    pub deleter: &'a D,
}

impl<'a, D> Operation<'a, D>
where
    D: Delete,
{
    pub fn execute(&self, id: WorkspaceScopeId) -> Result<()> {
        self.deleter.delete(id)
    }
}
