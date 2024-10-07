use crate::{entities::workspace::Entity, Result};

pub trait List {
    fn list(&self, parameters: ListParameters) -> Result<Vec<Entity>>;
}

pub struct Operation<'a, L>
where
    L: List,
{
    pub lister: &'a L,
}

pub struct Parameters<'a> {
    pub name_contains: Option<&'a str>,
}

pub struct ListParameters<'a> {
    pub name_contains: Option<&'a str>,
}

impl<'a, L> Operation<'a, L>
where
    L: List,
{
    pub fn execute(&self, parameters: Parameters) -> Result<Vec<Entity>> {
        let Parameters { name_contains } = parameters;

        self.lister.list(ListParameters { name_contains })
    }
}
