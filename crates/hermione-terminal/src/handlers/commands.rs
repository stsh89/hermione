use crate::{
    coordinator::ListCommandsWithinWorkspaceInput, themes::Theme, CommandPresenter, Coordinator,
    CreateWorkspaceCommandParams, DeleteWorkspaceCommandParams, EditWorkspaceCommandModel,
    EditWorkspaceCommandModelParameters, EditWorkspaceCommandParams, ListWorkspaceCommandsModel,
    ListWorkspaceCommandsModelParameters, ListWorkspaceCommandsParams, NewWorkspaceCommandModel,
    NewWorkspaceCommandModelParameters, NewWorkspaceCommandParams, Result,
    UpdateWorkspaceCommandParams, WorkspacePresenter, LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
};

pub struct CommandsHandler<'a> {
    pub coordinator: &'a Coordinator<'a>,
    pub theme: Theme,
}

impl<'a> CommandsHandler<'a> {
    pub fn create(&self, parameters: CreateWorkspaceCommandParams) -> Result<CommandPresenter> {
        let CreateWorkspaceCommandParams {
            workspace_id,
            name,
            program,
        } = parameters;

        let command = CommandPresenter {
            workspace_id,
            id: String::new(),
            name,
            program,
        };

        self.coordinator.create_command(command)
    }

    pub fn delete(&self, parameters: DeleteWorkspaceCommandParams) -> Result<WorkspacePresenter> {
        let DeleteWorkspaceCommandParams {
            workspace_id,
            command_id,
        } = parameters;

        self.coordinator
            .delete_command_from_workspace(&workspace_id, &command_id)?;

        self.coordinator.get_workspace(&workspace_id)
    }

    pub fn edit(self, parameters: EditWorkspaceCommandParams) -> Result<EditWorkspaceCommandModel> {
        let EditWorkspaceCommandParams {
            command_id,
            workspace_id,
        } = parameters;

        let command = self
            .coordinator
            .get_command_from_workspace(&workspace_id, &command_id)?;

        let workspace = self.coordinator.get_workspace(&workspace_id)?;

        EditWorkspaceCommandModel::new(EditWorkspaceCommandModelParameters {
            command,
            workspace,
            theme: self.theme,
        })
    }

    pub fn list(
        self,
        parameters: ListWorkspaceCommandsParams,
    ) -> Result<ListWorkspaceCommandsModel> {
        let ListWorkspaceCommandsParams {
            page_number,
            page_size,
            powershell_no_exit,
            search_query,
            workspace_id,
        } = parameters;

        let workspace = self.coordinator.get_workspace(&workspace_id)?;

        let commands =
            self.coordinator
                .list_commands_within_workspace(ListCommandsWithinWorkspaceInput {
                    workspace_id: &workspace_id,
                    program_contains: &search_query,
                    page_number,
                    page_size,
                })?;

        ListWorkspaceCommandsModel::new(ListWorkspaceCommandsModelParameters {
            commands,
            page_number,
            page_size,
            powershell_no_exit,
            search_query,
            workspace,
            theme: self.theme,
        })
    }

    pub fn new_command(
        self,
        parameters: NewWorkspaceCommandParams,
    ) -> Result<NewWorkspaceCommandModel> {
        let NewWorkspaceCommandParams { workspace_id } = parameters;

        let workspace = self.coordinator.get_workspace(&workspace_id)?;

        NewWorkspaceCommandModel::new(NewWorkspaceCommandModelParameters {
            workspace,
            theme: self.theme,
        })
    }

    pub fn update(
        self,
        parameters: UpdateWorkspaceCommandParams,
    ) -> Result<ListWorkspaceCommandsModel> {
        let UpdateWorkspaceCommandParams {
            command_id,
            workspace_id,
            name,
            program,
        } = parameters;

        let command = CommandPresenter {
            workspace_id,
            id: command_id.clone(),
            name,
            program,
        };

        let command = self.coordinator.update_command(command)?;
        let workspace = self.coordinator.get_workspace(&command.workspace_id)?;

        let commands =
            self.coordinator
                .list_commands_within_workspace(ListCommandsWithinWorkspaceInput {
                    page_number: 0,
                    page_size: LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
                    program_contains: &command.program,
                    workspace_id: &command.workspace_id,
                })?;

        let model = ListWorkspaceCommandsModel::new(ListWorkspaceCommandsModelParameters {
            commands,
            workspace,
            search_query: command.program,
            page_number: 0,
            page_size: LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
            powershell_no_exit: false,
            theme: self.theme,
        })?;

        Ok(model)
    }
}
