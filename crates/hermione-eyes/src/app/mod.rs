mod handlers;

use crate::{
    clients::memories,
    models::Model,
    router::{
        CreateCommandParameters, CreateWorkspaceParameters, DeleteCommandParameters,
        DeleteWorkspaceParameters, EditCommandParameters, EditWorkspaceParameters,
        ExecuteCommandParameters, GetCommandParameters, GetWorkspaceParameters,
        ListWorkspacesParameters, NewCommandParameters, Router, UpdateCommandParameters,
        UpdateWorkspaceParameters,
    },
    Result,
};
use ratatui::{backend::Backend, Terminal};
use tracing::instrument;

pub struct App {
    model: Box<dyn Model>,
    memories: memories::Client,
}

pub struct AppParameters {
    pub memories: memories::Client,
}

impl App {
    fn create_command(&mut self, parameters: CreateCommandParameters) -> Result<Box<dyn Model>> {
        let handler = handlers::create_command::Handler {
            memories: &mut self.memories,
        };

        let model = handler.handle(parameters)?;

        Ok(Box::new(model))
    }

    fn create_workspace(
        &mut self,
        parameters: CreateWorkspaceParameters,
    ) -> Result<Box<dyn Model>> {
        let handler = handlers::create_workspace::Handler {
            memories: &mut self.memories,
        };

        let model = handler.handle(parameters)?;

        Ok(Box::new(model))
    }

    fn delete_command(&mut self, parameters: DeleteCommandParameters) -> Result<Box<dyn Model>> {
        let model = handlers::delete_command::Handler {
            memories: &mut self.memories,
        }
        .handle(parameters)?;

        Ok(Box::new(model))
    }

    fn delete_workspace(
        &mut self,
        parameters: DeleteWorkspaceParameters,
    ) -> Result<Box<dyn Model>> {
        let model = handlers::delete_workspace::Handler {
            memories: &mut self.memories,
        }
        .handle(parameters)?;

        Ok(Box::new(model))
    }

    fn edit_command(&mut self, parameters: EditCommandParameters) -> Result<Box<dyn Model>> {
        let model = handlers::edit_command::Handler {
            memories: &self.memories,
        }
        .handle(parameters)?;

        Ok(Box::new(model))
    }

    fn edit_workspace(&mut self, parameters: EditWorkspaceParameters) -> Result<Box<dyn Model>> {
        let model = handlers::edit_workspace::Handler {
            memories: &self.memories,
        }
        .handle(parameters)?;

        Ok(Box::new(model))
    }

    fn execute_command(&mut self, parameters: ExecuteCommandParameters) -> Result<()> {
        handlers::execute_command::Handler {
            memories: &mut self.memories,
        }
        .handle(parameters)
    }

    fn update_command(&mut self, parameters: UpdateCommandParameters) -> Result<Box<dyn Model>> {
        let model = handlers::update_command::Handler {
            memories: &self.memories,
        }
        .handle(parameters)?;

        Ok(Box::new(model))
    }

    fn update_workspace(
        &mut self,
        parameters: UpdateWorkspaceParameters,
    ) -> Result<Box<dyn Model>> {
        let handler = handlers::update_workspace::Handler {
            memories: &mut self.memories,
        };

        let model = handler.handle(parameters)?;

        Ok(Box::new(model))
    }

    fn get_command(&mut self, parameters: GetCommandParameters) -> Result<Box<dyn Model>> {
        let handler = handlers::get_command::Handler {
            memories: &mut self.memories,
        };

        let model = handler.handle(parameters)?;

        Ok(Box::new(model))
    }

    fn get_workspace(&mut self, parameters: GetWorkspaceParameters) -> Result<Box<dyn Model>> {
        let handler = handlers::get_workspace::Handler {
            memories: &mut self.memories,
        };

        let model = handler.handle(parameters)?;

        Ok(Box::new(model))
    }

    fn new_command(&self, parameters: NewCommandParameters) -> Result<Box<dyn Model>> {
        let model = handlers::new_command::Handler {
            memories: &self.memories,
        }
        .handle(parameters)?;

        Ok(Box::new(model))
    }

    fn new_workspace(&self) -> Box<dyn Model> {
        let model = handlers::new_workspace::Handler {}.handle();

        Box::new(model)
    }

    fn list_workspaces(&self, parameters: ListWorkspacesParameters) -> Result<Box<dyn Model>> {
        let handler = handlers::list_workspaces::Handler {
            memories: &self.memories,
        };

        let model = handler.handle(parameters)?;

        Ok(Box::new(model))
    }

    fn handle(&mut self, route: Router) -> Result<()> {
        let model: Box<dyn Model> = match route.clone() {
            Router::CreateCommand(parameters) => self.create_command(parameters)?,
            Router::CreateWorkspace(parameters) => self.create_workspace(parameters)?,
            Router::DeleteCommand(parameters) => self.delete_command(parameters)?,
            Router::DeleteWorkspace(parameters) => self.delete_workspace(parameters)?,
            Router::EditCommand(parameters) => self.edit_command(parameters)?,
            Router::EditWorkspace(parameters) => self.edit_workspace(parameters)?,
            Router::ExecuteCommand(parameters) => {
                self.execute_command(parameters)?;

                return Ok(());
            }
            Router::GetCommand(parameters) => self.get_command(parameters)?,
            Router::GetWorkspace(parameters) => self.get_workspace(parameters)?,
            Router::ListWorkspaces(parameters) => self.list_workspaces(parameters)?,
            Router::NewCommand(parameters) => self.new_command(parameters)?,
            Router::NewWorkspace => self.new_workspace(),
            Router::UpdateCommand(parameters) => self.update_command(parameters)?,
            Router::UpdateWorkspace(parameters) => self.update_workspace(parameters)?,
        };

        self.model = model;

        Ok(())
    }

    pub fn new(parameters: AppParameters) -> Result<Self> {
        let AppParameters { memories } = parameters;
        let mut memories = memories;

        let workspaces = memories.list_workspaces()?;

        let model: Box<dyn Model> = if workspaces.is_empty() {
            let model = handlers::new_workspace::Handler {}.handle();
            Box::new(model)
        } else {
            let handler = handlers::get_workspace::Handler {
                memories: &mut memories,
            };

            let model = handler.handle(GetWorkspaceParameters {
                id: workspaces[0].id().to_string(),
                commands_search_query: String::new(),
            })?;

            Box::new(model)
        };

        Ok(Self { model, memories })
    }

    #[instrument(skip_all)]
    pub fn run(mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        tracing::info!("App started");

        while self.model.is_running() {
            terminal.draw(|f| self.model.view(f))?;

            let mut maybe_message = self.model.handle_event()?;

            while let Some(message) = maybe_message {
                maybe_message = self.model.update(message)?;
            }

            if let Some(route) = self.model.redirect() {
                self.handle(route)?;
            }
        }

        tracing::info!("App stopped");

        Ok(())
    }
}
