use crate::types::{
    command::{Entity, WorkspaceScopeId},
    shared::Result,
};

pub trait Get {
    fn get(&self, id: WorkspaceScopeId) -> Result<Entity>;
}

pub struct Operation<'a, R> {
    pub getter: &'a R,
}

impl<'a, R> Operation<'a, R>
where
    R: Get,
{
    pub fn execute(&self, id: WorkspaceScopeId) -> Result<Entity> {
        self.getter.get(id)
    }
}
