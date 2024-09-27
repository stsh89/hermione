use crate::types::{
    command::Entity,
    shared::{Result, ScopedId},
};

pub trait Get {
    fn get(&self, id: ScopedId) -> Result<Entity>;
}

pub struct Operation<'a, R> {
    pub getter: &'a R,
}

impl<'a, R> Operation<'a, R>
where
    R: Get,
{
    pub fn execute(&self, id: ScopedId) -> Result<Entity> {
        self.getter.get(id)
    }
}
