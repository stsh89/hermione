use crate::{
    definitions::{Command, WorkspaceId},
    services::{FilterCommandsParameters, ListCommands},
    Error, Result,
};

pub struct ListCommandsOperation<'a, L> {
    pub provider: &'a L,
}

pub struct ListCommandsParameters<'a> {
    pub page_size: u32,
    pub page_number: u32,
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

        if page_number == 0 {
            return Err(Error::InvalidArgument(
                "Page number must be greater than 0".to_string(),
            ));
        }

        if page_size == 0 {
            return Err(Error::InvalidArgument(
                "Page size must be greater than 0".to_string(),
            ));
        }

        self.provider.list_commands(FilterCommandsParameters {
            program_contains,
            page_number,
            page_size,
            workspace_id,
        })
    }
}
