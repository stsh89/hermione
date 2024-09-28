use crate::types::{workspace::Entity, Result};

pub trait List {
    fn list(&self) -> Result<Vec<Entity>>;
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
    pub fn execute(&self) -> Result<Vec<Entity>> {
        self.lister.list()
    }
}
