use crate::{
    coordinator::{Coordinator, ListWorkspacesInput},
    CreateWorkspaceParams, DeleteWorkspaceParams, EditWorkspaceModel, EditWorkspaceModelParameters,
    EditWorkspaceParams, ListWorkspaceModelParameters, ListWorkspacesModel, ListWorkspacesParams,
    NewWorkspaceModel, Result, UpdateWorkspaceParams, WorkspacePresenter,
    LIST_WORKSPACES_PAGE_SIZE,
};

pub struct WorkspacesHandler<'a> {
    pub coordinator: &'a Coordinator<'a>,
}

impl<'a> WorkspacesHandler<'a> {
    pub fn create(self, parameters: CreateWorkspaceParams) -> Result<ListWorkspacesModel> {
        let CreateWorkspaceParams { name, location } = parameters;

        self.coordinator.create_workspace(WorkspacePresenter {
            id: String::new(),
            location,
            name: name.clone(),
        })?;

        let workspaces = self.coordinator.list_workspaces(ListWorkspacesInput {
            name_contains: name.as_str(),
            page_number: 0,
            page_size: LIST_WORKSPACES_PAGE_SIZE,
        })?;

        let model = ListWorkspacesModel::new(ListWorkspaceModelParameters {
            workspaces,
            search_query: name,
            page_number: 0,
            page_size: LIST_WORKSPACES_PAGE_SIZE,
        })?;

        Ok(model)
    }

    pub fn delete(self, parameters: DeleteWorkspaceParams) -> Result<ListWorkspacesModel> {
        let DeleteWorkspaceParams { id } = parameters;

        self.coordinator.delete_workspace(&id)?;

        let workspaces = self.coordinator.list_workspaces(ListWorkspacesInput {
            name_contains: "",
            page_number: 0,
            page_size: LIST_WORKSPACES_PAGE_SIZE,
        })?;

        let model = ListWorkspacesModel::new(ListWorkspaceModelParameters {
            workspaces,
            search_query: String::new(),
            page_number: 0,
            page_size: LIST_WORKSPACES_PAGE_SIZE,
        })?;

        Ok(model)
    }

    pub fn edit(self, parameters: EditWorkspaceParams) -> Result<EditWorkspaceModel> {
        let EditWorkspaceParams { id } = parameters;

        let workspace = self.coordinator.get_workspace(&id)?;

        EditWorkspaceModel::new(EditWorkspaceModelParameters { workspace })
    }

    pub fn list(self, parameters: ListWorkspacesParams) -> Result<ListWorkspacesModel> {
        let ListWorkspacesParams {
            search_query,
            page_number,
            page_size,
        } = parameters;

        let workspaces = self.coordinator.list_workspaces(ListWorkspacesInput {
            name_contains: &search_query,
            page_number,
            page_size,
        })?;

        ListWorkspacesModel::new(ListWorkspaceModelParameters {
            workspaces,
            search_query,
            page_number,
            page_size,
        })
    }

    pub fn new_workspace(self) -> Result<NewWorkspaceModel> {
        NewWorkspaceModel::new()
    }

    pub fn update(&self, parameters: UpdateWorkspaceParams) -> Result<WorkspacePresenter> {
        let UpdateWorkspaceParams { id, name, location } = parameters;

        let mut workspace = self.coordinator.get_workspace(&id)?;

        workspace.name = name;
        workspace.location = location;

        self.coordinator.update_workspace(workspace)
    }
}
