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
    pub program_contains: Option<&'a str>,
    pub workspace_id: Id,
}

pub struct ListParameters<'a> {
    pub workspace_id: Id,
    pub program_contains: Option<&'a str>,
}

impl<'a, L> Operation<'a, L>
where
    L: List,
{
    pub fn execute(&self, parameters: Parameters) -> Result<Vec<Entity>> {
        let Parameters {
            program_contains,
            workspace_id,
        } = parameters;

        self.lister.list(ListParameters {
            program_contains,
            workspace_id,
        })
    }
}
