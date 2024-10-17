use crate::{
    entities::{command::Entity, command::ScopedId},
    Result,
};

pub trait Find {
    fn find(&self, id: ScopedId) -> Result<Option<Entity>>;
}

pub struct Operation<'a, R> {
    pub finder: &'a R,
}

impl<'a, R> Operation<'a, R>
where
    R: Find,
{
    pub fn execute(&self, id: ScopedId) -> Result<Option<Entity>> {
        self.finder.find(id)
    }
}
