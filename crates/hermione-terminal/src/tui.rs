use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyEvent, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Position, Rect},
    widgets::Widget,
    Frame, Terminal,
};
use std::{
    io::{stdout, Stdout},
    panic,
};

type Result<T> = anyhow::Result<T>;

pub type BoxedModel<R, M> = Box<dyn Model<Route = R, Message = M>>;

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

pub trait Router {
    type Route;
    type Message;

    fn default_model(&self) -> Result<BoxedModel<Self::Route, Self::Message>>;
    fn handle(&self, route: Self::Route) -> Result<Option<BoxedModel<Self::Route, Self::Message>>>;
}

pub struct EventHandler<F, T>
where
    F: Fn(event::KeyEvent) -> Option<T>,
{
    f: F,
}

#[derive(Default)]
pub struct Input {
    value: String,
    character_index: usize,
}

pub fn run<R, M>(router: impl Router<Route = R, Message = M>) -> Result<()> {
    install_panic_hook();

    let mut terminal = init_terminal()?;
    let mut model = router.default_model()?;

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

    restore_terminal()
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

impl Input {
    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.value
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.value.len())
    }

    pub fn character_index(&self) -> usize {
        self.character_index
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.value.chars().count())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.value.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.value.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.value = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn delete_all_chars(&mut self) {
        self.value.clear();
        self.character_index = 0;
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.value.insert(index, new_char);
        self.move_cursor_right();
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn new(value: String) -> Self {
        let mut input = Self::default();

        for c in value.chars() {
            input.enter_char(c);
        }

        input
    }

    pub fn render<W>(&self, frame: &mut Frame, area: Rect, widget: W)
    where
        W: Widget,
    {
        frame.render_widget(widget, area);

        frame.set_cursor_position(Position::new(
            area.x + self.character_index() as u16 + 1,
            area.y + 1,
        ));
    }

    pub fn secret_value(&self) -> String {
        "*".repeat(self.value.chars().count())
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    Ok(terminal)
}

fn install_panic_hook() {
    let original_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        stdout().execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

fn restore_terminal() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
