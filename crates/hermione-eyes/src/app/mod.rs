pub mod helpers;

mod message;
mod router;

use crate::{clients::memories, routes, types::Result};
use ratatui::{backend::Backend, crossterm::event, Frame, Terminal};
use tracing::instrument;

pub use message::*;
pub use router::*;

pub trait Hook {
    fn handle_event(&self) -> Result<Option<Message>> {
        EventHandler::new(|key_event| key_event.try_into().ok()).handle_event()
    }

    fn is_running(&self) -> bool {
        true
    }

    fn redirect(&mut self) -> Option<Router> {
        None
    }

    fn update(&mut self, _message: Message) -> Result<Option<Message>> {
        Ok(None)
    }

    fn view(&mut self, _frame: &mut Frame) {}
}

pub struct App {
    model: Box<dyn Hook>,
    memories: memories::Client,
}

pub struct AppParameters {
    pub memories: memories::Client,
}

struct EventHandler<F>
where
    F: Fn(event::KeyEvent) -> Option<Message>,
{
    f: F,
}

impl App {
    fn copy_to_clipboard(&mut self, parameters: CopyToClipboardParameters) -> Result<()> {
        let handler = routes::workspaces::commands::copy_to_clipboard::Handler {
            memories: &self.memories,
        };

        handler.handle(parameters)
    }

    fn create_command(&mut self, parameters: CreateCommandParameters) -> Result<Box<dyn Hook>> {
        let handler = routes::workspaces::commands::create::Handler {
            memories: &mut self.memories,
        };

        let model = handler.handle(parameters)?;

        Ok(Box::new(model))
    }

    fn create_workspace(&mut self, parameters: CreateWorkspaceParameters) -> Result<Box<dyn Hook>> {
        let handler = routes::workspaces::create::Handler {
            memories: &mut self.memories,
        };

        let model = handler.handle(parameters)?;

        Ok(Box::new(model))
    }

    fn delete_command(&mut self, parameters: DeleteCommandParameters) -> Result<Box<dyn Hook>> {
        let model = routes::workspaces::commands::delete::Handler {
            memories: &mut self.memories,
        }
        .handle(parameters)?;

        Ok(Box::new(model))
    }

    fn delete_workspace(&mut self, parameters: DeleteWorkspaceParameters) -> Result<Box<dyn Hook>> {
        let model = routes::workspaces::delete::Handler {
            memories: &mut self.memories,
        }
        .handle(parameters)?;

        Ok(Box::new(model))
    }

    fn edit_command(&mut self, parameters: EditCommandParameters) -> Result<Box<dyn Hook>> {
        let model = routes::workspaces::commands::edit::Handler {
            memories: &self.memories,
        }
        .handle(parameters)?;

        Ok(Box::new(model))
    }

    fn edit_workspace(&mut self, parameters: EditWorkspaceParameters) -> Result<Box<dyn Hook>> {
        let model = routes::workspaces::edit::Handler {
            memories: &self.memories,
        }
        .handle(parameters)?;

        Ok(Box::new(model))
    }

    fn execute_command(&mut self, parameters: ExecuteCommandParameters) -> Result<()> {
        routes::workspaces::commands::execute::Handler {
            memories: &mut self.memories,
        }
        .handle(parameters)
    }

    fn update_command(&mut self, parameters: UpdateCommandParameters) -> Result<Box<dyn Hook>> {
        let model = routes::workspaces::commands::update::Handler {
            memories: &self.memories,
        }
        .handle(parameters)?;

        Ok(Box::new(model))
    }

    fn update_workspace(&mut self, parameters: UpdateWorkspaceParameters) -> Result<Box<dyn Hook>> {
        let handler = routes::workspaces::update::Handler {
            memories: &mut self.memories,
        };

        let model = handler.handle(parameters)?;

        Ok(Box::new(model))
    }

    fn get_command(&mut self, parameters: GetCommandParameters) -> Result<Box<dyn Hook>> {
        let handler = routes::workspaces::commands::get::Handler {
            memories: &mut self.memories,
        };

        let model = handler.handle(parameters)?;

        Ok(Box::new(model))
    }

    fn get_workspace(&mut self, parameters: GetWorkspaceParameters) -> Result<Box<dyn Hook>> {
        let handler = routes::workspaces::get::Handler {
            memories: &mut self.memories,
        };

        let model = handler.handle(parameters)?;

        Ok(Box::new(model))
    }

    fn new_command(&self, parameters: NewCommandParameters) -> Result<Box<dyn Hook>> {
        let model = routes::workspaces::commands::new::Handler {
            memories: &self.memories,
        }
        .handle(parameters)?;

        Ok(Box::new(model))
    }

    fn new_workspace(&self) -> Box<dyn Hook> {
        let model = routes::workspaces::new::Handler {}.handle();

        Box::new(model)
    }

    fn list_workspaces(&self, parameters: ListWorkspacesParameters) -> Result<Box<dyn Hook>> {
        let handler = routes::workspaces::list::Handler {
            memories: &self.memories,
        };

        let model = handler.handle(parameters)?;

        Ok(Box::new(model))
    }

    fn handle(&mut self, route: Router) -> Result<()> {
        let model: Box<dyn Hook> = match route {
            Router::CopyToClipboard(parameters) => {
                self.copy_to_clipboard(parameters)?;

                return Ok(());
            }
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

        let model: Box<dyn Hook> = if workspaces.is_empty() {
            let model = routes::workspaces::new::Handler {}.handle();
            Box::new(model)
        } else {
            let handler = routes::workspaces::get::Handler {
                memories: &mut memories,
            };

            let model = handler.handle(GetWorkspaceParameters {
                id: workspaces[0].id.clone(),
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

impl<F> EventHandler<F>
where
    F: Fn(event::KeyEvent) -> Option<Message>,
{
    fn new(f: F) -> Self {
        Self { f }
    }

    fn handle_event(self) -> Result<Option<Message>> {
        let tui_event = event::read()?;
        tracing::info!(tui_event = ?tui_event);

        if let event::Event::Key(key) = tui_event {
            if key.kind == event::KeyEventKind::Press {
                let message = (self.f)(key);

                return Ok(message);
            }
        }

        Ok(None)
    }
}
