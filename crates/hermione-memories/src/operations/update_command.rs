use crate::types::{
    command::Entity,
    shared::{Id, Result},
};

pub trait Update {
    fn update(&self, workspace_id: Id, command: &Entity) -> Result<()>;
}

pub struct Operation<'a, U> {
    pub updater: &'a U,
}

impl<'a, U> Operation<'a, U>
where
    U: Update,
{
    pub fn execute(&self, workspace_id: Id, command: &Entity) -> Result<()> {
        self.updater.update(workspace_id, command)
    }
}
