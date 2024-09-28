use crate::types::{command::ScopedId, Result};

pub trait Delete {
    fn delete(&self, id: ScopedId) -> Result<()>;
}

pub struct Operation<'a, D> {
    pub deleter: &'a D,
}

impl<'a, D> Operation<'a, D>
where
    D: Delete,
{
    pub fn execute(&self, id: ScopedId) -> Result<()> {
        self.deleter.delete(id)
    }
}
