use crate::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyEvent, KeyEventKind},
    Frame,
};

pub trait Model {
    type Route;
    type Message;

    fn handle_event(&self) -> Result<Option<Self::Message>>;

    fn is_running(&self) -> bool {
        true
    }

    fn redirect(&mut self) -> Option<Self::Route> {
        None
    }

    fn update(&mut self, _message: Self::Message) -> Result<Option<Self::Message>> {
        Ok(None)
    }

    fn view(&mut self, _frame: &mut Frame) {}
}

type BoxedModel<R, M> = Box<dyn Model<Route = R, Message = M>>;

pub trait Router {
    type Route;
    type Message;

    fn handle(&self, route: Self::Route) -> Result<Option<BoxedModel<Self::Route, Self::Message>>>;
}

pub struct EventHandler<F, T>
where
    F: Fn(event::KeyEvent) -> Option<T>,
{
    f: F,
}

impl<F, T> EventHandler<F, T>
where
    F: Fn(KeyEvent) -> Option<T>,
{
    pub fn new(f: F) -> Self {
        Self { f }
    }

    pub fn handle_event(self) -> Result<Option<T>> {
        let tui_event = event::read()?;

        if let Event::Key(key) = tui_event {
            if key.kind == KeyEventKind::Press {
                let message = (self.f)(key);

                return Ok(message);
            }
        }

        Ok(None)
    }
}
