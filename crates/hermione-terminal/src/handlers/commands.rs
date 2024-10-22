use crate::{
    CommandPresenter, Coordinator, CreateWorkspaceCommandParameters,
    DeleteWorkspaceCommandParameters, EditWorkspaceCommandModel,
    EditWorkspaceCommandModelParameters, EditWorkspaceCommandParameters,
    ListCommandsWithinWorkspaceFilter, ListWorkspaceCommandsModel,
    ListWorkspaceCommandsModelParameters, ListWorkspaceCommandsParameters,
    NewWorkspaceCommandModel, NewWorkspaceCommandModelParameters, NewWorkspaceCommandParameters,
    Result, UpdateWorkspaceCommandParameters, WorkspacePresenter,
    LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
};

pub struct CommandsHandler<'a> {
    pub coordinator: &'a Coordinator,
}

impl<'a> CommandsHandler<'a> {
    pub fn create(&self, parameters: CreateWorkspaceCommandParameters) -> Result<CommandPresenter> {
        let CreateWorkspaceCommandParameters {
            workspace_id,
            name,
            program,
        } = parameters;

        self.coordinator.commands().create(CommandPresenter {
            workspace_id: workspace_id.clone(),
            id: String::new(),
            name,
            program: program.clone(),
        })
    }

    pub fn delete(
        &self,
        parameters: DeleteWorkspaceCommandParameters,
    ) -> Result<WorkspacePresenter> {
        let DeleteWorkspaceCommandParameters {
            workspace_id,
            command_id,
        } = parameters;

        self.coordinator
            .commands()
            .delete(&workspace_id, &command_id)?;
        self.coordinator.workspaces().get(&workspace_id)
    }

    pub fn edit(
        self,
        parameters: EditWorkspaceCommandParameters,
    ) -> Result<EditWorkspaceCommandModel> {
        let EditWorkspaceCommandParameters {
            command_id,
            workspace_id,
        } = parameters;

        let command = self
            .coordinator
            .commands()
            .get(&workspace_id, &command_id)?;

        let workspace = self.coordinator.workspaces().get(&workspace_id)?;

        EditWorkspaceCommandModel::new(EditWorkspaceCommandModelParameters { command, workspace })
    }

    pub fn list(
        self,
        parameters: ListWorkspaceCommandsParameters,
    ) -> Result<ListWorkspaceCommandsModel> {
        let ListWorkspaceCommandsParameters {
            page_number,
            page_size,
            powershell_no_exit,
            search_query,
            workspace_id,
        } = parameters;

        let workspace = self.coordinator.workspaces().get(&workspace_id)?;

        let commands = self
            .coordinator
            .commands()
            .list(ListCommandsWithinWorkspaceFilter {
                workspace_id: &workspace_id,
                program_contains: &search_query,
                page_number,
                page_size,
            })?;

        let workspace = self.coordinator.workspaces().track_access_time(workspace)?;

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
        parameters: NewWorkspaceCommandParameters,
    ) -> Result<NewWorkspaceCommandModel> {
        let NewWorkspaceCommandParameters { workspace_id } = parameters;

        let workspace = self.coordinator.workspaces().get(&workspace_id)?;

        NewWorkspaceCommandModel::new(NewWorkspaceCommandModelParameters { workspace })
    }

    pub fn update(
        self,
        parameters: UpdateWorkspaceCommandParameters,
    ) -> Result<ListWorkspaceCommandsModel> {
        let UpdateWorkspaceCommandParameters {
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

        let command = self.coordinator.commands().update(command)?;
        let workspace = self.coordinator.workspaces().get(&command.workspace_id)?;
        let commands = self
            .coordinator
            .commands()
            .list(ListCommandsWithinWorkspaceFilter {
                page_number: 0,
                page_size: LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
                program_contains: &command.program,
                workspace_id: &workspace.id,
            })?;

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
