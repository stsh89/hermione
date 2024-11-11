use crate::{
    coordinator::{Coordinator, ListWorkspacesInput},
    models::{WorkspaceModel, WorkspacesModel, WorkspacesModelParameters},
    themes::Theme,
    CreateWorkspaceParams, DeleteWorkspaceParams, EditWorkspaceParams, ListWorkspacesParams,
    Result, UpdateWorkspaceParams, WorkspacePresenter,
};
use uuid::Uuid;

pub struct WorkspacesHandler<'a> {
    pub coordinator: &'a Coordinator,
    pub theme: Theme,
}

impl<'a> WorkspacesHandler<'a> {
    pub fn create(self, parameters: CreateWorkspaceParams) -> Result<WorkspacesModel> {
        let CreateWorkspaceParams { name, location } = parameters;

        self.coordinator.create_workspace(WorkspacePresenter {
            id: Uuid::nil(),
            location,
            name: name.clone(),
        })?;

        let workspaces = self.coordinator.list_workspaces(ListWorkspacesInput {
            name_contains: name.as_str(),
            page_number: None,
            page_size: None,
        })?;

        let model = WorkspacesModel::new(WorkspacesModelParameters {
            workspaces,
            search_query: name,
            page_number: None,
            page_size: None,
            theme: self.theme,
        })?;

        Ok(model)
    }

    pub fn delete(self, parameters: DeleteWorkspaceParams) -> Result<WorkspacesModel> {
        let DeleteWorkspaceParams { id } = parameters;

        self.coordinator.delete_workspace(id)?;

        let workspaces = self.coordinator.list_workspaces(ListWorkspacesInput {
            name_contains: "",
            page_number: None,
            page_size: None,
        })?;

        let model = WorkspacesModel::new(WorkspacesModelParameters {
            workspaces,
            search_query: String::new(),
            page_number: None,
            page_size: None,
            theme: self.theme,
        })?;

        Ok(model)
    }

    pub fn edit(self, parameters: EditWorkspaceParams) -> Result<WorkspaceModel> {
        let EditWorkspaceParams { id } = parameters;

        let workspace = self.coordinator.get_workspace(id)?;

        Ok(WorkspaceModel::default()
            .workspace(workspace)
            .theme(self.theme))
    }

    pub fn list(self, parameters: ListWorkspacesParams) -> Result<WorkspacesModel> {
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

        WorkspacesModel::new(WorkspacesModelParameters {
            workspaces,
            search_query,
            page_number,
            page_size,
            theme: self.theme,
        })
    }

    pub fn new_workspace(self) -> Result<WorkspaceModel> {
        Ok(WorkspaceModel::default().theme(self.theme))
    }

    pub fn update(&self, parameters: UpdateWorkspaceParams) -> Result<WorkspacePresenter> {
        let UpdateWorkspaceParams { id, name, location } = parameters;

        let mut workspace = self.coordinator.get_workspace(id)?;

        workspace.name = name;
        workspace.location = location;

        self.coordinator.update_workspace(workspace)
    }
}
