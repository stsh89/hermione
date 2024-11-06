use std::num::NonZeroU32;

use crate::{
    definitions::{Command, WorkspaceId},
    services::{FilterCommandsParameters, ListCommands, StorageProvider},
    Result,
};

pub struct ListCommandsOperation<'a, SP>
where
    SP: StorageProvider,
{
    pub provider: &'a SP,
}

pub struct ListCommandsParameters<'a> {
    pub page_size: NonZeroU32,
    pub page_number: NonZeroU32,
    pub program_contains: Option<&'a str>,
    pub workspace_id: Option<&'a WorkspaceId>,
}

impl<'a, L> ListCommandsOperation<'a, L>
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

        self.provider.list_commands(FilterCommandsParameters {
            program_contains,
            page_number: Into::<u32>::into(page_number) - 1,
            page_size: page_size.into(),
            workspace_id,
        })
    }
}
