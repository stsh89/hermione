use std::num::NonZeroU32;

use crate::{
    definitions::Workspace,
    services::{FilterWorkspacesParameters, ListWorkspaces, StorageService},
    Result,
};

const DEFAULT_PAGE_NUMBER: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(1) };
const DEFAULT_PAGE_SIZE: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100) };

pub struct ListWorkspacesOperation<'a, SP>
where
    SP: StorageService,
{
    pub provider: &'a SP,
}

pub struct ListWorkspacesParameters<'a> {
    pub name_contains: Option<&'a str>,
    pub page_number: Option<NonZeroU32>,
    pub page_size: Option<NonZeroU32>,
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

        let page_number = page_number.unwrap_or(DEFAULT_PAGE_NUMBER).get() - 1;
        let page_size = page_size.unwrap_or(DEFAULT_PAGE_SIZE).get();

        self.provider.list_workspaces(FilterWorkspacesParameters {
            name_contains,
            page_number,
            page_size,
        })
    }
}
