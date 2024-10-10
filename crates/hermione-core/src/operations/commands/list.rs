use crate::{entities::command::Entity, Result};

pub trait List {
    fn list(&self, parameters: ListParameters) -> Result<Vec<Entity>>;
}

pub struct Operation<'a, L>
where
    L: List,
{
    pub lister: &'a L,
}

pub struct Parameters {
    pub page_number: u32,
    pub page_size: u32,
}

pub struct ListParameters {
    pub page_number: u32,
    pub page_size: u32,
}

impl<'a, L> Operation<'a, L>
where
    L: List,
{
    pub fn execute(&self, parameters: Parameters) -> Result<Vec<Entity>> {
        let Parameters {
            page_number,
            page_size,
        } = parameters;

        self.lister.list(ListParameters {
            page_number,
            page_size,
        })
    }
}
