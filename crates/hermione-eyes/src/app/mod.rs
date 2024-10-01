mod message;

use crate::{
    clients::memories,
    routes::{self, Router, RouterParameters},
    Result,
};
use ratatui::{backend::Backend, crossterm::event, Frame, Terminal};
use tracing::instrument;

pub use message::*;

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
    fn handle(&self, router: Router) -> Result<Option<Box<dyn Hook>>> {
        router.handle(RouterParameters {
            memories: &self.memories,
        })
    }

    fn initial_route(&self) -> Result<Router> {
        let workspaces = self.memories.list_workspaces()?;

        let router = if workspaces.is_empty() {
            Router::Workspaces(routes::workspaces::Router::New)
        } else {
            routes::workspaces::commands::list::Parameters {
                workspace_id: workspaces[0].id.clone(),
                search_query: String::new(),
            }
            .into()
        };

        Ok(router)
    }

    pub fn new(parameters: AppParameters) -> Result<Self> {
        let AppParameters { memories } = parameters;

        Ok(Self { memories })
    }

    #[instrument(skip_all)]
    pub fn run(self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        tracing::info!("App started");

        let route = self.initial_route()?;

        let Some(mut model) = self.handle(route)? else {
            return Ok(());
        };

        while model.is_running() {
            terminal.draw(|f| model.view(f))?;

            let mut maybe_message = model.handle_event()?;

            while let Some(message) = maybe_message {
                maybe_message = model.update(message)?;
            }

            if let Some(router) = model.redirect() {
                if let Some(change) = self.handle(router)? {
                    model = change;
                }
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

        if let event::Event::Key(key) = tui_event {
            if key.kind == event::KeyEventKind::Press {
                let message = (self.f)(key);

                return Ok(message);
            }
        }

        Ok(None)
    }
}
