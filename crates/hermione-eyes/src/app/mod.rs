mod handlers;

use crate::{
    clients::{self, organizer},
    models::{
        GetWorkspaceModel, GetWorkspaceModelParameters, ListWorkspacesModel,
        ListWorkspacesModelParameters, Model,
    },
    router::{
        CommandPaletteParameters, CreateCommandParameters, CreateWorkspaceParameters,
        GetCommandParameters, GetWorkspaceParameters, ListWorkspacesParameters, Router,
    },
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct App {
    route: Router,
    model: Box<dyn Model>,
    organizer: clients::organizer::Client,
}

pub struct AppParameters {
    pub organizer: clients::organizer::Client,
}

impl App {
    fn command_palette(&self, parameters: CommandPaletteParameters) -> Result<Box<dyn Model>> {
        let model = handlers::command_palette::Handler {
            parameters,
            route: self.route.clone(),
        }
        .handle()?;

        Ok(Box::new(model))
    }

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
            Router::CommandPalette(parameters) => self.command_palette(parameters)?,
            Router::CreateCommand(parameters) => self.create_command(parameters)?,
            Router::CreateWorkspace(parameters) => self.create_workspace(parameters)?,
            Router::DeleteCommand => self.delete_command()?,
            Router::DeleteWorkspace => self.delete_workspace()?,
            Router::GetCommand(parameters) => self.get_command(parameters)?,
            Router::GetWorkspace(parameters) => self.get_workspace(parameters)?,
            Router::ListWorkspaces(parameters) => self.list_workspaces(parameters)?,
            Router::NewCommand => self.new_command(),
            Router::NewWorkspace => self.new_workspace(),
        };

        self.model = model;
        self.route = route;

        Ok(())
    }

    pub fn new(parameters: AppParameters) -> Result<Self> {
        let AppParameters { organizer } = parameters;
        let mut organizer = organizer;

        let workspaces = organizer.list_workspaces();

        let route = if workspaces.is_empty() {
            Router::NewWorkspace
        } else {
            Router::GetWorkspace(GetWorkspaceParameters {
                number: 0,
                commands_search_query: String::new(),
            })
        };

        let model: Box<dyn Model> = match route.clone() {
            Router::NewWorkspace => {
                let model = handlers::new_workspace::Handler {}.handle();
                Box::new(model)
            }
            Router::GetWorkspace(parameters) => {
                let model = handlers::get_workspace::Handler {
                    organizer: &mut organizer,
                    parameters,
                }
                .handle()?;
                Box::new(model)
            }
            _ => unreachable!(),
        };

        Ok(Self {
            model,
            route,
            organizer,
        })
    }

    pub fn run(mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        while self.model.is_running() {
            terminal.draw(|f| self.model.view(f))?;

            let mut maybe_message = self.model.handle_event()?;

            while let Some(message) = maybe_message {
                maybe_message = self.model.update(message)?;
            }

            if let Some(route) = self.model.redirect() {
                self.handle(route.clone())?;
            }
        }

        self.organizer.save()?;

        Ok(())
    }
}
