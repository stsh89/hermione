use hermione_coordinator::ListCommandsWithinWorkspaceInput;

use crate::{
    Command, Coordinator, CreateWorkspaceCommandParams, DeleteWorkspaceCommandParams,
    EditWorkspaceCommandModel, EditWorkspaceCommandModelParameters, EditWorkspaceCommandParams,
    ListWorkspaceCommandsModel, ListWorkspaceCommandsModelParameters, ListWorkspaceCommandsParams,
    NewWorkspaceCommandModel, NewWorkspaceCommandModelParameters, NewWorkspaceCommandParams,
    Result, UpdateWorkspaceCommandParams, Workspace, LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
};

pub struct CommandsHandler<'a> {
    pub coordinator: &'a Coordinator,
}

impl<'a> CommandsHandler<'a> {
    pub fn create(&self, parameters: CreateWorkspaceCommandParams) -> Result<Command> {
        let CreateWorkspaceCommandParams {
            workspace_id,
            name,
            program,
        } = parameters;

        let dto = Command {
            workspace_id,
            id: String::new(),
            name,
            program,
        }
        .into();

        let dto = self.coordinator.create_command(dto)?;

        Ok(dto.into())
    }

    pub fn delete(&self, parameters: DeleteWorkspaceCommandParams) -> Result<Workspace> {
        let DeleteWorkspaceCommandParams {
            workspace_id,
            command_id,
        } = parameters;

        self.coordinator
            .delete_command_from_workspace(&workspace_id, &command_id)?;
        let dto = self.coordinator.get_workspace(&workspace_id)?;

        Ok(dto.into())
    }

    pub fn edit(self, parameters: EditWorkspaceCommandParams) -> Result<EditWorkspaceCommandModel> {
        let EditWorkspaceCommandParams {
            command_id,
            workspace_id,
        } = parameters;

        let command = self
            .coordinator
            .get_command_from_workspace(&workspace_id, &command_id)?
            .into();

        let workspace = self.coordinator.get_workspace(&workspace_id)?.into();

        EditWorkspaceCommandModel::new(EditWorkspaceCommandModelParameters { command, workspace })
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

        let workspace = self.coordinator.get_workspace(&workspace_id)?.into();

        let commands = self
            .coordinator
            .list_commands_within_workspace(ListCommandsWithinWorkspaceInput {
                workspace_id: &workspace_id,
                program_contains: &search_query,
                page_number,
                page_size,
            })?
            .into_iter()
            .map(Into::into)
            .collect();

        ListWorkspaceCommandsModel::new(ListWorkspaceCommandsModelParameters {
            commands,
            page_number,
            page_size,
            powershell_no_exit,
            search_query,
            workspace,
        })
    }

    pub fn new_command(
        self,
        parameters: NewWorkspaceCommandParams,
    ) -> Result<NewWorkspaceCommandModel> {
        let NewWorkspaceCommandParams { workspace_id } = parameters;

        let workspace = self.coordinator.get_workspace(&workspace_id)?.into();

        NewWorkspaceCommandModel::new(NewWorkspaceCommandModelParameters { workspace })
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

        let command = Command {
            workspace_id,
            id: command_id.clone(),
            name,
            program,
        };

        let command = self.coordinator.update_command(command.into())?;
        let workspace = self
            .coordinator
            .get_workspace(&command.workspace_id)?
            .into();
        let commands = self
            .coordinator
            .list_commands_within_workspace(ListCommandsWithinWorkspaceInput {
                page_number: 0,
                page_size: LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
                program_contains: &command.program,
                workspace_id: &command.workspace_id,
            })?
            .into_iter()
            .map(Into::into)
            .collect();

        let model = ListWorkspaceCommandsModel::new(ListWorkspaceCommandsModelParameters {
            commands,
            workspace,
            search_query: command.program,
            page_number: 0,
            page_size: LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
            powershell_no_exit: false,
        })?;

        Ok(model)
    }
}
