use crate::{entities::command::Entity, Id, Result};

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
    pub page_number: u32,
    pub page_size: u32,
    pub program_contains: &'a str,
    pub workspace_id: Id,
}

pub struct ListParameters<'a> {
    pub page_number: u32,
    pub page_size: u32,
    pub program_contains: &'a str,
    pub workspace_id: Id,
}

impl<'a, L> Operation<'a, L>
where
    L: List,
{
    pub fn execute(&self, parameters: Parameters) -> Result<Vec<Entity>> {
        let Parameters {
            page_number,
            page_size,
            program_contains,
            workspace_id,
        } = parameters;

        self.lister.list(ListParameters {
            page_number,
            page_size,
            program_contains,
            workspace_id,
        })
    }
}
