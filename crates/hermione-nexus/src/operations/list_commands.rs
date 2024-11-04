use crate::{
    definitions::{Command, WorkspaceId},
    services::{FilterCommandsParameters, ListCommands, StorageProvider},
    Error, Result,
};

pub struct ListCommandsOperation<'a, SP>
where
    SP: StorageProvider,
{
    pub provider: &'a SP,
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

        self.validate_parameters(&parameters)?;

        let ListCommandsParameters {
            page_size,
            page_number,
            program_contains,
            workspace_id,
        } = parameters;

        self.provider.list_commands(FilterCommandsParameters {
            program_contains,
            page_number,
            page_size,
            workspace_id,
        })
    }

    fn validate_page_number(&self, page_number: u32) -> Result<()> {
        if page_number == 0 {
            return Err(Error::InvalidArgument(
                "Page number must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    fn validate_page_size(&self, page_size: u32) -> Result<()> {
        if page_size == 0 {
            return Err(Error::InvalidArgument(
                "Page size must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    fn validate_parameters(&self, parameters: &ListCommandsParameters) -> Result<()> {
        let ListCommandsParameters {
            page_size,
            page_number,
            program_contains: _,
            workspace_id: _,
        } = parameters;

        self.validate_page_number(*page_number)?;
        self.validate_page_size(*page_size)?;

        Ok(())
    }
}
