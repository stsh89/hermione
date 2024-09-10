use super::{
    elements::{Input, Selector},
    TableauModel, TableauModelParameters,
};
use crate::{
    clients::{CommandExecutor, CommandExecutorOutput, CreateCommandParameters, OrganizerClient},
    data::{Command, Workspace},
    Result,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Position},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState, Paragraph},
    Frame,
};

enum ActiveElement {
    Selector,
    SearchBar,
}

pub enum Message {
    ActivateSearchBar,
    CreateCommand(NewCommand),
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
    active_element: ActiveElement,
    organizer: &'a mut OrganizerClient,
    search_bar: Input,
    selector: Selector<Command>,
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
    Exited,
    Running,
}

struct View<'a> {
    active_element: &'a ActiveElement,
    search_bar: &'a Input,
    selector: &'a Selector<Command>,
    workspace: &'a Workspace,
}

impl ActiveElement {
    fn is_search_bar(&self) -> bool {
        matches!(self, ActiveElement::SearchBar)
    }

    fn toggle(&mut self) {
        *self = match *self {
            ActiveElement::Selector => ActiveElement::SearchBar,
            ActiveElement::SearchBar => ActiveElement::Selector,
        };
    }
}

impl<'a> Model<'a> {
    fn create_command(&mut self, parameters: NewCommand) -> Result<()> {
        let NewCommand { name, program } = parameters;

        self.organizer.create_command(CreateCommandParameters {
            workspace_id: self.workspace.id,
            name,
            program,
        })?;

        self.reload_workspace()?;
        self.reset_selector();

        Ok(())
    }

    pub fn delete_command(&mut self) -> Result<()> {
        let Some(command) = self.selector.item() else {
            return Ok(());
        };

        self.organizer
            .delete_command(self.workspace.id, command.id)?;
        self.reload_workspace()?;
        self.reset_selector();

        Ok(())
    }

    pub fn delete_char(&mut self) {
        if self.active_element.is_search_bar() {
            self.search_bar.delete_char();
            self.update_selector();
        }
    }

    pub fn enter_char(&mut self, new_char: char) {
        if self.active_element.is_search_bar() {
            self.search_bar.enter_char(new_char);
            self.update_selector();
        }
    }

    fn exit(&mut self) {
        self.state = State::Exited;
    }

    pub fn has_selected_command(&self) -> bool {
        self.selector.item().is_some()
    }

    pub fn in_editor_mode(&self) -> bool {
        matches!(self.active_element, ActiveElement::SearchBar)
    }

    pub fn in_normal_mode(&self) -> bool {
        matches!(self.active_element, ActiveElement::Selector)
    }

    pub fn is_exited(&self) -> bool {
        matches!(self.state, State::Exited)
    }

    fn move_cursor_left(&mut self) {
        if self.active_element.is_search_bar() {
            self.search_bar.move_cursor_left();
        }
    }

    fn move_cursor_right(&mut self) {
        if self.active_element.is_search_bar() {
            self.search_bar.move_cursor_right();
        }
    }

    pub fn new(params: ModelParameters<'a>) -> Self {
        let ModelParameters {
            organizer,
            workspace,
        } = params;

        let mut selector = Selector::new(workspace.commands.clone());
        selector.select_first();

        Self {
            active_element: ActiveElement::Selector,
            organizer,
            search_bar: Input::default(),
            selector,
            state: State::Running,
            workspace,
        }
    }

    fn activate_search_bar(&mut self) {
        self.active_element = ActiveElement::SearchBar;
        self.selector.unselect();
    }

    fn reload_workspace(&mut self) -> Result<()> {
        self.workspace = self.organizer.get_workspace(self.workspace.id)?;

        Ok(())
    }

    fn reset_selector(&mut self) {
        self.selector = Selector::new(self.workspace.commands.clone());
    }

    fn select_next(&mut self) {
        match self.active_element {
            ActiveElement::Selector => self.selector.next(),
            ActiveElement::SearchBar => {
                self.selector.select_first();
                self.active_element.toggle();
            }
        }
    }

    fn select_prev(&mut self) {
        match self.active_element {
            ActiveElement::Selector => self.selector.prev(),
            ActiveElement::SearchBar => {
                self.selector.select_last();
                self.active_element.toggle();
            }
        }
    }

    pub fn tableau(&mut self) -> Result<Option<TableauModel>> {
        let Some(command) = self.selector.item() else {
            return Ok(None);
        };

        let CommandExecutorOutput { stdout, stderr } = CommandExecutor::new(command).execute()?;
        let model = TableauModel::new(TableauModelParameters {
            command,
            stdout,
            stderr,
        });

        Ok(Some(model))
    }

    pub fn update(&mut self, message: Message) -> Result<()> {
        match message {
            Message::CreateCommand(parameters) => self.create_command(parameters)?,
            Message::DeleteChar => self.delete_char(),
            Message::DeleteCommand => self.delete_command()?,
            Message::EnterChar(c) => self.enter_char(c),
            Message::Exit => self.exit(),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::SelectNextCommand => self.select_next(),
            Message::SelectPreviousCommand => self.select_prev(),
            Message::ActivateSearchBar => self.activate_search_bar(),
        }

        Ok(())
    }

    fn update_selector(&mut self) {
        let search_query = self.search_bar.value();
        self.selector = Selector::new(
            self.workspace
                .commands
                .clone()
                .into_iter()
                .filter(|command| command.program.contains(search_query))
                .collect(),
        );
    }

    pub fn view(&self, frame: &mut Frame) {
        let view = View {
            workspace: &self.workspace,
            selector: &self.selector,
            active_element: &self.active_element,
            search_bar: &self.search_bar,
        };

        view.render(frame);
    }
}

impl<'a> View<'a> {
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

        let values: Vec<&str> = self
            .selector
            .items()
            .iter()
            .map(|command| command.program.as_str())
            .collect();

        let list = List::new(values)
            .highlight_style(Style::new().reversed())
            .block(
                Block::new()
                    .title(format!("{} commands", self.workspace.name))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::all()),
            );
        let mut state = ListState::default();

        state.select(self.selector.item_number());

        frame.render_stateful_widget(list, programs_list, &mut state);

        let paragraph = Paragraph::new(
            self.selector
                .item()
                .map(|command| command.name.as_str())
                .unwrap_or_default(),
        )
        .block(
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
