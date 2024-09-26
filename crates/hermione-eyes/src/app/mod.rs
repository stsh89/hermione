mod handlers;

use crate::{
    clients,
    models::Model,
    router::{
        CreateCommandParameters, CreateWorkspaceParameters, ExecuteCommandParameters,
        GetCommandParameters, GetWorkspaceParameters, ListWorkspacesParameters, Router,
        UpdateCommandParameters, UpdateWorkspaceParameters,
    },
    Result,
};
use ratatui::{backend::Backend, Terminal};
use tracing::instrument;

pub struct App {
    model: Box<dyn Model>,
    organizer: clients::organizer::Client,
}

pub struct AppParameters {
    pub organizer: clients::organizer::Client,
}

impl App {
    fn create_command(&mut self, parameters: CreateCommandParameters) -> Result<Box<dyn Model>> {
        let model = handlers::create_command::Handler {
            organizer: &mut self.organizer,
            parameters,
        }
        .handle()?;

        Ok(Box::new(model))
    }

    fn create_workspace(
        &mut self,
        parameters: CreateWorkspaceParameters,
    ) -> Result<Box<dyn Model>> {
        let model = handlers::create_workspace::Handler {
            organizer: &mut self.organizer,
            parameters,
        }
        .handle()?;

        Ok(Box::new(model))
    }

    fn delete_command(&mut self) -> Result<Box<dyn Model>> {
        let model = handlers::delete_command::Handler {
            organizer: &mut self.organizer,
        }
        .handle()?;

        Ok(Box::new(model))
    }

    fn delete_workspace(&mut self) -> Result<Box<dyn Model>> {
        let model = handlers::delete_workspace::Handler {
            organizer: &mut self.organizer,
        }
        .handle()?;

        Ok(Box::new(model))
    }

    fn edit_command(&mut self) -> Result<Box<dyn Model>> {
        let command = self.organizer.get_command(0, 0)?;

        let model = handlers::edit_command::Handler { command }.handle();

        Ok(Box::new(model))
    }

    fn edit_workspace(&mut self) -> Result<Box<dyn Model>> {
        let workspace = self.organizer.get_workspace(0)?;

        let model = handlers::edit_workspace::Handler { workspace }.handle();

        Ok(Box::new(model))
    }

    fn execute_command(&mut self, parameters: ExecuteCommandParameters) -> Result<()> {
        handlers::execute_command::Handler {
            parameters,
            organizer: &mut self.organizer,
        }
        .handle()
    }

    fn update_command(&mut self, parameters: UpdateCommandParameters) -> Result<Box<dyn Model>> {
        let model = handlers::update_command::Handler {
            organizer: &mut self.organizer,
            parameters,
        }
        .handle()?;

        Ok(Box::new(model))
    }

    fn update_workspace(
        &mut self,
        parameters: UpdateWorkspaceParameters,
    ) -> Result<Box<dyn Model>> {
        let model = handlers::update_workspace::Handler {
            organizer: &mut self.organizer,
            parameters,
        }
        .handle()?;

        Ok(Box::new(model))
    }

    fn get_command(&mut self, parameters: GetCommandParameters) -> Result<Box<dyn Model>> {
        let model = handlers::get_command::Handler {
            organizer: &mut self.organizer,
            parameters,
        }
        .handle()?;

        Ok(Box::new(model))
    }

    fn get_workspace(&mut self, parameters: GetWorkspaceParameters) -> Result<Box<dyn Model>> {
        let model = handlers::get_workspace::Handler {
            organizer: &mut self.organizer,
            parameters,
        }
        .handle()?;

        Ok(Box::new(model))
    }

    fn new_command(&self) -> Box<dyn Model> {
        let model = handlers::new_command::Handler {}.handle();

        Box::new(model)
    }

    fn new_workspace(&self) -> Box<dyn Model> {
        let model = handlers::new_workspace::Handler {}.handle();

        Box::new(model)
    }

    fn list_workspaces(&self, parameters: ListWorkspacesParameters) -> Result<Box<dyn Model>> {
        let model = handlers::list_workspaces::Handler {
            organizer: &self.organizer,
            parameters,
        }
        .handle()?;

        Ok(Box::new(model))
    }

    fn handle(&mut self, route: Router) -> Result<()> {
        let model: Box<dyn Model> = match route.clone() {
            Router::CreateCommand(parameters) => self.create_command(parameters)?,
            Router::CreateWorkspace(parameters) => self.create_workspace(parameters)?,
            Router::DeleteCommand => self.delete_command()?,
            Router::DeleteWorkspace => self.delete_workspace()?,
            Router::EditCommand => self.edit_command()?,
            Router::EditWorkspace => self.edit_workspace()?,
            Router::ExecuteCommand(parameters) => {
                self.execute_command(parameters)?;

                return Ok(());
            }
            Router::GetCommand(parameters) => self.get_command(parameters)?,
            Router::GetWorkspace(parameters) => self.get_workspace(parameters)?,
            Router::ListWorkspaces(parameters) => self.list_workspaces(parameters)?,
            Router::NewCommand => self.new_command(),
            Router::NewWorkspace => self.new_workspace(),
            Router::UpdateCommand(parameters) => self.update_command(parameters)?,
            Router::UpdateWorkspace(parameters) => self.update_workspace(parameters)?,
        };

        self.model = model;

        Ok(())
    }

    pub fn new(parameters: AppParameters) -> Result<Self> {
        let AppParameters { organizer } = parameters;
        let mut organizer = organizer;

        let workspaces = organizer.list_workspaces();

        let model: Box<dyn Model> = if workspaces.is_empty() {
            let model = handlers::new_workspace::Handler {}.handle();

            Box::new(model)
        } else {
            let model = handlers::get_workspace::Handler {
                organizer: &mut organizer,
                parameters: GetWorkspaceParameters {
                    number: 0,
                    commands_search_query: String::new(),
                },
            }
            .handle()?;

            Box::new(model)
        };

        Ok(Self { model, organizer })
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

        self.organizer.save()?;
        tracing::info!("App stopped");

        Ok(())
    }
}
