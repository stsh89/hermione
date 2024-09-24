use crate::{
    models::{handle_event, highlight_style, shared::Input, Message},
    router::Router,
    Result,
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Alignment, Constraint, Direction, Layout, Position},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub const NEW_WORKSPACE: &str = "New workspace";
pub const NEW_COMMAND: &str = "New command";
pub const DELETE_WORKSPACE: &str = "Delete workspace";
pub const RENAME_WORKSPACE: &str = "Rename workspace";

pub struct CommandPaletteModel {
    actions: Vec<Action>,
    is_running: bool,
    redirect: Option<Router>,
    back: Router,
    input: Input,
    list_state: ListState,
}

pub struct CommandPaletteModelParameters {
    pub commands: Vec<Action>,
    pub back: Router,
}

#[derive(Clone, Copy)]
pub enum Action {
    NewWorkspace,
    NewCommand,
    DeleteWorkspace,
}

impl Action {
    pub fn as_str(&self) -> &'static str {
        match self {
            Action::NewWorkspace => NEW_WORKSPACE,
            Action::NewCommand => NEW_COMMAND,
            Action::DeleteWorkspace => DELETE_WORKSPACE,
        }
    }
}

impl CommandPaletteModel {
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn new(parameters: CommandPaletteModelParameters) -> Self {
        let CommandPaletteModelParameters { commands, back } = parameters;

        Self {
            actions: commands,
            is_running: true,
            redirect: None,
            back,
            input: Input::active(),
            list_state: ListState::default(),
        }
    }

    pub fn handle_event(&self) -> Result<Option<Message>> {
        handle_event(message)
    }

    pub fn redirect(&self) -> Option<&Router> {
        self.redirect.as_ref()
    }

    fn select_next(&mut self) {
        self.list_state.select_next();
    }

    fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    fn enter_char(&mut self, c: char) {
        self.input.enter_char(c);
    }

    fn delete_char(&mut self) {
        self.input.delete_char();
    }

    fn delete_all_chars(&mut self) {
        self.input.delete_all_chars();
    }

    fn move_cursor_left(&mut self) {
        self.input.move_cursor_left();
    }

    fn move_cursor_right(&mut self) {
        self.input.move_cursor_right();
    }

    fn sumbit(&mut self) {
        let search_query = self.input.value().to_lowercase();
        let mut commands = self.actions.clone();

        commands = if search_query.is_empty() {
            commands
        } else {
            commands
                .into_iter()
                .filter(|command| command.as_str().to_lowercase().contains(&search_query))
                .collect()
        };

        let Some(command) = self
            .list_state
            .selected()
            .and_then(|index| commands.get(index))
        else {
            return;
        };

        match command {
            Action::NewWorkspace => self.redirect = Some(Router::NewWorkspace),
            Action::NewCommand => self.redirect = Some(Router::NewCommand),
            Action::DeleteWorkspace => self.redirect = Some(Router::DeleteWorkspace),
        }
    }

    fn redirect_back(&mut self) {
        self.redirect = Some(self.back.clone());
    }

    pub fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::Back => self.redirect_back(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::Exit => self.is_running = false,
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::SelectNext => self.select_next(),
            Message::SelectPrevious => self.select_previous(),
            Message::Sumbit => self.sumbit(),
            _ => {}
        }

        Ok(None)
    }

    pub fn view(&mut self, frame: &mut Frame) {
        let [header, search, commands] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(1),
                Constraint::Max(3),
                Constraint::Min(3),
            ])
            .areas(frame.area());

        let paragraph = Paragraph::new("Welcome to command palette").alignment(Alignment::Center);
        frame.render_widget(paragraph, header);

        let block = Block::default().borders(Borders::all()).title("Search");
        let paragraph = Paragraph::new(self.input.value()).block(block);

        frame.render_widget(paragraph, search);
        frame.set_cursor_position(Position::new(
            search.x + self.input.character_index() as u16 + 1,
            search.y + 1,
        ));

        let block = Block::default().borders(Borders::all()).title("Actions");
        let search_query = self.input.value().to_lowercase();

        let items: Vec<ListItem> = if search_query.is_empty() {
            self.actions.iter().map(ListItem::from).collect()
        } else {
            self.actions
                .iter()
                .filter(|command| command.as_str().to_lowercase().contains(&search_query))
                .map(ListItem::from)
                .collect()
        };

        let list = List::new(items)
            .block(block)
            .highlight_style(highlight_style());

        frame.render_stateful_widget(list, commands, &mut self.list_state);
    }
}

fn message(key_event: KeyEvent) -> Option<Message> {
    let message = match key_event.code {
        KeyCode::Char(c) => Message::EnterChar(c),
        KeyCode::Backspace => match key_event.modifiers {
            KeyModifiers::CONTROL => Message::DeleteAllChars,
            _ => Message::DeleteChar,
        },
        KeyCode::Esc => Message::Back,
        KeyCode::Enter => Message::Sumbit,
        KeyCode::Left => Message::MoveCusorLeft,
        KeyCode::Right => Message::MoveCusorRight,
        KeyCode::Up => Message::SelectPrevious,
        KeyCode::Down => Message::SelectNext,
        _ => return None,
    };

    Some(message)
}

impl<'a> From<&Action> for ListItem<'a> {
    fn from(command: &Action) -> Self {
        ListItem::new(command.as_str().to_string())
    }
}

impl TryFrom<String> for Action {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            NEW_WORKSPACE => Ok(Action::NewWorkspace),
            NEW_COMMAND => Ok(Action::NewCommand),
            DELETE_WORKSPACE => Ok(Action::DeleteWorkspace),
            _ => Err(anyhow::anyhow!("Unknown command: {}", value)),
        }
    }
}
