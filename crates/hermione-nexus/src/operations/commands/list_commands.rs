use crate::{
    definitions::{Command, WorkspaceId},
    services::{FilterCommandsParameters, ListCommands, StorageService},
    Result,
};
use std::num::NonZeroU32;

const DEFAULT_PAGE_NUMBER: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(1) };
const DEFAULT_PAGE_SIZE: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100) };

pub struct ListCommandsOperation<'a, SP>
where
    SP: StorageService,
{
    pub provider: &'a SP,
}

pub struct ListCommandsParameters<'a> {
    pub page_size: Option<NonZeroU32>,
    pub page_number: Option<NonZeroU32>,
    pub program_contains: Option<&'a str>,
    pub workspace_id: Option<WorkspaceId>,
}

impl<L> ListCommandsOperation<'_, L>
where
    L: ListCommands,
{
    pub fn execute(&self, parameters: ListCommandsParameters) -> Result<Vec<Command>> {
        tracing::info!(operation = "List commands");

        let ListCommandsParameters {
            page_size,
            page_number,
            program_contains,
            workspace_id,
        } = parameters;

        let page_number = page_number.unwrap_or(DEFAULT_PAGE_NUMBER).get() - 1;
        let page_size = page_size.unwrap_or(DEFAULT_PAGE_SIZE).get();

        self.provider.list_commands(FilterCommandsParameters {
            program_contains,
            page_number,
            page_size,
            workspace_id,
        })
    }
}
