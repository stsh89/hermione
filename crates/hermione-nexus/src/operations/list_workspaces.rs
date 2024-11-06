use std::num::NonZeroU32;

use crate::{
    definitions::Workspace,
    services::{FilterWorkspacesParameters, ListWorkspaces, StorageProvider},
    Result,
};

pub struct ListWorkspacesOperation<'a, SP>
where
    SP: StorageProvider,
{
    pub provider: &'a SP,
}

pub struct ListWorkspacesParameters<'a> {
    pub name_contains: Option<&'a str>,
    pub page_number: NonZeroU32,
    pub page_size: NonZeroU32,
}

impl<'a, L> ListWorkspacesOperation<'a, L>
where
    L: ListWorkspaces,
{
    pub fn execute(&self, parameters: ListWorkspacesParameters) -> Result<Vec<Workspace>> {
        tracing::info!(operation = "List workspaces");

        let ListWorkspacesParameters {
            name_contains,
            page_number,
            page_size,
        } = parameters;

        self.provider.list_workspaces(FilterWorkspacesParameters {
            name_contains,
            page_number: Into::<u32>::into(page_number) - 1,
            page_size: page_size.into(),
        })
    }
}
