use crate::{
    coordinator::ListCommandsWithinWorkspaceInput,
    models::{
        EditCommandModel, EditCommandModelParameters, ListWorkspaceCommandsModel,
        ListWorkspaceCommandsModelParameters, NewWorkspaceCommandModel,
        NewWorkspaceCommandModelParameters,
    },
    themes::Theme,
    CommandPresenter, Coordinator, CreateWorkspaceCommandParams, DeleteCommandParams,
    EditCommandParams, ListWorkspaceCommandsParams, NewWorkspaceCommandParams, Result,
    UpdateWorkspaceCommandParams, WorkspacePresenter,
};
use uuid::Uuid;

pub struct CommandsHandler<'a> {
    pub coordinator: &'a Coordinator,
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
            id: Uuid::nil(),
            name,
            program,
        };

        self.coordinator.create_command(command)
    }

    pub fn delete(&self, parameters: DeleteCommandParams) -> Result<WorkspacePresenter> {
        let DeleteCommandParams {
            command_id,
            workspace_id,
        } = parameters;

        self.coordinator.delete_command(command_id)?;

        self.coordinator.get_workspace(workspace_id)
    }

    pub fn edit(self, parameters: EditCommandParams) -> Result<EditCommandModel> {
        let EditCommandParams { command_id } = parameters;

        let command = self.coordinator.get_command(command_id)?;
        let workspace = self.coordinator.get_workspace(command.workspace_id)?;

        EditCommandModel::new(EditCommandModelParameters {
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

        let workspace = self.coordinator.get_workspace(workspace_id)?;

        let commands =
            self.coordinator
                .list_workspace_commands(ListCommandsWithinWorkspaceInput {
                    workspace_id,
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

        let workspace = self.coordinator.get_workspace(workspace_id)?;

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
            command_id: id,
            workspace_id,
            name,
            program,
        } = parameters;

        let command = CommandPresenter {
            workspace_id,
            id,
            name,
            program,
        };

        let command = self.coordinator.update_command(command)?;
        let workspace = self.coordinator.get_workspace(command.workspace_id)?;

        let commands =
            self.coordinator
                .list_workspace_commands(ListCommandsWithinWorkspaceInput {
                    page_number: None,
                    page_size: None,
                    program_contains: &command.program,
                    workspace_id: command.workspace_id,
                })?;

        let model = ListWorkspaceCommandsModel::new(ListWorkspaceCommandsModelParameters {
            commands,
            workspace,
            search_query: command.program,
            page_number: None,
            page_size: None,
            powershell_no_exit: false,
            theme: self.theme,
        })?;

        Ok(model)
    }
}
