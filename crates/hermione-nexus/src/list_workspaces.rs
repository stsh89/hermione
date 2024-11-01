use crate::{
    services::storage::{FilterWorkspacesParameters, ListWorkspaces, Workspace},
    Error, Result,
};

pub struct ListWorkspacesOperation<'a, L> {
    pub operator: &'a L,
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

        let ListWorkspacesParameters {
            name_contains,
            page_number,
            page_size,
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

        self.operator.list_workspaces(FilterWorkspacesParameters {
            name_contains,
            page_number,
            page_size,
        })
    }
}
