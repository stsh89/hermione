use crate::types::shared::{Id, Result};

pub trait Delete {
    fn delete(&self, workspace_id: Id, command_id: Id) -> Result<()>;
}

pub struct Operation<D> {
    pub deleter: D,
}

impl<D> Delete for Operation<D>
where
    D: Delete,
{
    fn delete(&self, workspace_id: Id, command_id: Id) -> Result<()> {
        self.deleter.delete(workspace_id, command_id)
    }
}
