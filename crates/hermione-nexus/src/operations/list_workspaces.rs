use crate::{
    definitions::Workspace,
    services::{FilterWorkspacesParameters, ListWorkspaces, StorageProvider},
    Error, Result,
};

pub struct ListWorkspacesOperation<'a, SP>
where
    SP: StorageProvider,
{
    pub provider: &'a SP,
}

pub struct ListWorkspacesParameters<'a> {
    pub name_contains: Option<&'a str>,
    pub page_number: u32,
    pub page_size: u32,
}

impl<'a, L> ListWorkspacesOperation<'a, L>
where
    L: ListWorkspaces,
{
    pub fn execute(&self, parameters: ListWorkspacesParameters) -> Result<Vec<Workspace>> {
        tracing::info!(operation = "List workspaces");

        self.validate_parameters(&parameters)?;

        let ListWorkspacesParameters {
            name_contains,
            page_number,
            page_size,
        } = parameters;

        self.provider.list_workspaces(FilterWorkspacesParameters {
            name_contains,
            page_number,
            page_size,
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

    fn validate_parameters(&self, parameters: &ListWorkspacesParameters) -> Result<()> {
        let ListWorkspacesParameters {
            name_contains: _,
            page_number,
            page_size,
        } = parameters;

        self.validate_page_number(*page_number)?;
        self.validate_page_size(*page_size)?;

        Ok(())
    }
}
