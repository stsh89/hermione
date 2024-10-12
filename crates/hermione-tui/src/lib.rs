mod input;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Frame, Terminal,
};
use std::{
    io::{stdout, Stdout},
    panic,
};

pub use input::Input;

type Result<T> = anyhow::Result<T>;

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

    fn handle_initial_route(&self) -> Result<Option<BoxedModel<Self::Route, Self::Message>>>;
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
    F: Fn(event::KeyEvent) -> Option<T>,
{
    pub fn new(f: F) -> Self {
        Self { f }
    }

    pub fn handle_event(self) -> Result<Option<T>> {
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

pub fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    Ok(terminal)
}

pub fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        stdout().execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

pub fn restore_terminal() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn run(router: impl Router) -> Result<()> {
    let mut terminal = init_terminal()?;

    let Some(mut model) = router.handle_initial_route()? else {
        return Ok(());
    };

    while model.is_running() {
        terminal.draw(|f| model.view(f))?;

        let mut maybe_message = model.handle_event()?;

        while let Some(message) = maybe_message {
            maybe_message = model.update(message)?;
        }

        if let Some(route) = model.redirect() {
            if let Some(change) = router.handle(route)? {
                model = change;
            }
        }
    }

    Ok(())
}
