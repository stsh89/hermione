use crate::{
    models::{helpers::Input, highlight_style, Message, Model},
    router::Router,
    Result,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub const NEW_WORKSPACE: &str = "New workspace";
pub const NEW_COMMAND: &str = "New command";
pub const DELETE_WORKSPACE: &str = "Delete workspace";
pub const RENAME_WORKSPACE: &str = "Rename workspace";
pub const DELETE_COMMAND: &str = "Delete command";

pub struct CommandPaletteModel {
    actions: Vec<Action>,
    redirect: Option<Router>,
    back: Router,
    input: Input,
    list_state: ListState,
}

pub struct CommandPaletteModelParameters {
    pub commands: Vec<Action>,
    pub back: Router,
}

impl Model for CommandPaletteModel {
    fn redirect(&self) -> Option<&Router> {
        self.redirect.as_ref()
    }

    fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::Back => self.back(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::SelectNext => self.select_next(),
            Message::SelectPrevious => self.select_previous(),
            Message::Submit => self.sumbit(),
            _ => {}
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
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

#[derive(Clone, Copy)]
pub enum Action {
    NewWorkspace,
    NewCommand,
    DeleteWorkspace,
    DeleteCommand,
}

impl Action {
    pub fn as_str(&self) -> &'static str {
        match self {
            Action::NewWorkspace => NEW_WORKSPACE,
            Action::NewCommand => NEW_COMMAND,
            Action::DeleteWorkspace => DELETE_WORKSPACE,
            Action::DeleteCommand => DELETE_COMMAND,
        }
    }
}

impl CommandPaletteModel {
    pub fn new(parameters: CommandPaletteModelParameters) -> Self {
        let CommandPaletteModelParameters { commands, back } = parameters;

        Self {
            actions: commands,
            redirect: None,
            back,
            input: Input::active(),
            list_state: ListState::default(),
        }
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
            Action::DeleteCommand => self.redirect = Some(Router::DeleteCommand),
        }
    }

    fn back(&mut self) {
        self.redirect = Some(self.back.clone());
    }
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
            DELETE_COMMAND => Ok(Action::DeleteCommand),
            _ => Err(anyhow::anyhow!("Unknown command: {}", value)),
        }
    }
}
