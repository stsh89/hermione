use super::elements::{Input, Selector};
use crate::{
    clients::OrganizerClient,
    data::{Command, Workspace},
    Result,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Position},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState, Paragraph},
    Frame,
};

enum Element {
    Selector,
    SearchBar,
}

pub enum Message {
    ActivateSearchBar,
    DeleteChar,
    DeleteCommand,
    EnterChar(char),
    Exit,
    MoveCusorLeft,
    MoveCusorRight,
    SelectNextCommand,
    SelectPreviousCommand,
}

pub struct Model<'a> {
    element: Element,
    organizer: &'a mut OrganizerClient,
    search_bar: Input,
    selector: Option<Selector<Command>>,
    state: State,
    workspace: Workspace,
}

pub struct ModelParameters<'a> {
    pub organizer: &'a mut OrganizerClient,
    pub workspace: Workspace,
}

pub struct NewCommand {
    pub name: String,
    pub program: String,
}

enum State {
    Editing,
    Exited,
    Selecting,
}

struct View<'a> {
    active_element: &'a Element,
    search_bar: &'a Input,
    selector: Option<&'a Selector<Command>>,
    workspace: &'a Workspace,
}

impl Element {
    fn is_search_bar(&self) -> bool {
        matches!(self, Element::SearchBar)
    }

    fn toggle(&mut self) {
        *self = match *self {
            Element::Selector => Element::SearchBar,
            Element::SearchBar => Element::Selector,
        };
    }
}

impl Message {
    fn is_idempotent(&self) -> bool {
        match &self {
            Self::ActivateSearchBar
            | Self::DeleteChar
            | Self::EnterChar(_)
            | Self::Exit
            | Self::MoveCusorLeft
            | Self::MoveCusorRight
            | Self::SelectNextCommand
            | Self::SelectPreviousCommand => true,
            Self::DeleteCommand => false,
        }
    }
}

impl<'a> Model<'a> {
    fn activate_search_bar(&mut self) {
        self.element = Element::SearchBar;
        self.state = State::Editing;
    }

    pub fn command(&self) -> Option<&Command> {
        self.selector.as_ref().map(|selector| selector.item())
    }

    fn delete_command(&mut self) -> Result<()> {
        if let Some(selector) = &self.selector {
            self.organizer
                .delete_command(self.workspace.id, selector.item().id)?;
        }

        Ok(())
    }

    fn delete_char(&mut self) {
        if self.element.is_search_bar() {
            self.search_bar.delete_char();
        }
    }

    fn enter_char(&mut self, new_char: char) {
        if self.element.is_search_bar() {
            self.search_bar.enter_char(new_char);
        }
    }

    fn exit(&mut self) {
        self.state = State::Exited;
    }

    pub fn is_editing(&self) -> bool {
        matches!(self.state, State::Editing)
    }

    pub fn is_exited(&self) -> bool {
        matches!(self.state, State::Exited)
    }

    pub fn is_selecting(&self) -> bool {
        matches!(self.state, State::Selecting)
    }

    fn move_cursor_left(&mut self) {
        if self.element.is_search_bar() {
            self.search_bar.move_cursor_left();
        }
    }

    fn move_cursor_right(&mut self) {
        if self.element.is_search_bar() {
            self.search_bar.move_cursor_right();
        }
    }

    pub fn new(params: ModelParameters<'a>) -> Result<Self> {
        let ModelParameters {
            organizer,
            workspace,
        } = params;

        let mut model = Self {
            element: Element::Selector,
            organizer,
            search_bar: Input::default(),
            selector: None,
            state: State::Selecting,
            workspace,
        };

        model.update_selector()?;

        Ok(model)
    }

    fn select_next_command(&mut self) {
        match self.element {
            Element::Selector => {
                if let Some(selector) = &mut self.selector {
                    selector.next();
                }
            }
            Element::SearchBar => {
                self.element.toggle();
                self.state = State::Selecting;
            }
        }
    }

    fn select_previous_command(&mut self) {
        match self.element {
            Element::Selector => {
                if let Some(selector) = &mut self.selector {
                    selector.previous();
                }
            }
            Element::SearchBar => {
                self.element.toggle();
                self.state = State::Selecting;
            }
        }
    }

    pub fn update(mut self, message: Message) -> Result<Self> {
        let is_idempotent = message.is_idempotent();

        match message {
            Message::ActivateSearchBar => self.activate_search_bar(),
            Message::DeleteChar => self.delete_char(),
            Message::DeleteCommand => self.delete_command()?,
            Message::EnterChar(c) => self.enter_char(c),
            Message::Exit => self.exit(),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::SelectNextCommand => self.select_next_command(),
            Message::SelectPreviousCommand => self.select_previous_command(),
        }

        let selector = if is_idempotent {
            self.selector
        } else {
            self.update_selector()?;
            self.selector
        };

        let model = Self { selector, ..self };

        Ok(model)
    }

    fn update_selector(&mut self) -> Result<()> {
        let commands = self.organizer.get_workspace(self.workspace.id)?.commands;

        let selector = if commands.is_empty() {
            None
        } else {
            Some(Selector::new(commands)?)
        };

        self.selector = selector;

        Ok(())
    }

    pub fn view(&self, frame: &mut Frame) {
        let view = View {
            workspace: &self.workspace,
            selector: self.selector.as_ref(),
            active_element: &self.element,
            search_bar: &self.search_bar,
        };

        view.render(frame);
    }

    pub fn workspace(&self) -> &Workspace {
        &self.workspace
    }
}

impl<'a> View<'a> {
    fn selected_command_name(&self) -> &str {
        self.selector
            .as_ref()
            .map(|selector| selector.item().name.as_str())
            .unwrap_or_default()
    }

    fn selected_command_number(&self) -> Option<usize> {
        self.selector
            .as_ref()
            .map(|selector| selector.item_number())
    }

    fn programs(&self) -> Vec<String> {
        let Some(selector) = self.selector else {
            return Vec::new();
        };

        selector
            .items()
            .iter()
            .map(|command| command.program.clone())
            .collect()
    }

    fn render(&self, frame: &mut Frame) {
        let layout = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Min(3),
                Constraint::Percentage(100),
                Constraint::Min(3),
            ],
        )
        .flex(Flex::Start);
        let [search_bar, programs_list, program_name] = layout.areas(frame.area());

        let list = List::new(self.programs())
            .highlight_style(Style::new().reversed())
            .block(
                Block::new()
                    .title(format!("{} commands", self.workspace.name))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::all()),
            );
        let mut state = ListState::default();

        state.select(self.selected_command_number());

        frame.render_stateful_widget(list, programs_list, &mut state);

        let paragraph = Paragraph::new(self.selected_command_name()).block(
            Block::new()
                .title("Command name")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(paragraph, program_name);

        let paragraph = Paragraph::new(self.search_bar.value()).block(
            Block::new()
                .title("Search")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(paragraph, search_bar);

        if self.active_element.is_search_bar() {
            frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                search_bar.x + self.search_bar.character_index() as u16 + 1,
                // Move one line down, from the border to the input line
                search_bar.y + 1,
            ));
        }
    }
}
