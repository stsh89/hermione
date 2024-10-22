use crate::{
    Coordinator, CreateWorkspaceParameters, DeleteWorkspaceParameters, EditWorkspaceModel,
    EditWorkspaceModelParameters, EditWorkspaceParameters, ListWorkspaceModelParameters,
    ListWorkspacesFilter, ListWorkspacesModel, ListWorkspacesParameters, NewWorkspaceModel, Result,
    UpdateWorkspaceParameters, WorkspacePresenter, LIST_WORKSPACES_PAGE_SIZE,
};

pub struct WorkspacesHandler<'a> {
    pub coordinator: &'a Coordinator,
}

impl<'a> WorkspacesHandler<'a> {
    pub fn create(self, parameters: CreateWorkspaceParameters) -> Result<ListWorkspacesModel> {
        let CreateWorkspaceParameters { name, location } = parameters;

        self.coordinator.workspaces().create(WorkspacePresenter {
            id: String::new(),
            location,
            name: name.clone(),
        })?;

        let workspaces = self.coordinator.workspaces().list(ListWorkspacesFilter {
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

    pub fn delete(self, parameters: DeleteWorkspaceParameters) -> Result<ListWorkspacesModel> {
        let DeleteWorkspaceParameters { id } = parameters;

        self.coordinator.workspaces().delete(&id)?;
        let workspaces = self.coordinator.workspaces().list(ListWorkspacesFilter {
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

    pub fn edit(self, parameters: EditWorkspaceParameters) -> Result<EditWorkspaceModel> {
        let EditWorkspaceParameters { id } = parameters;

        let workspace = self.coordinator.workspaces().get(&id)?;

        EditWorkspaceModel::new(EditWorkspaceModelParameters { workspace })
    }

    pub fn list(self, parameters: ListWorkspacesParameters) -> Result<ListWorkspacesModel> {
        let ListWorkspacesParameters {
            search_query,
            page_number,
            page_size,
        } = parameters;

        let workspaces = self.coordinator.workspaces().list(ListWorkspacesFilter {
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

    pub fn update(&self, parameters: UpdateWorkspaceParameters) -> Result<WorkspacePresenter> {
        let UpdateWorkspaceParameters { id, name, location } = parameters;

        let mut workspace = self.coordinator.workspaces().get(&id)?;

        workspace.name = name;
        workspace.location = location;

        self.coordinator.workspaces().update(workspace)
    }
}
