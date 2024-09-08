use super::{elements::Input, TableauModel, TableauModelParameters};
use crate::{
    clients::{CommandExecutor, CommandExecutorOutput, OrganizerClient},
    data::Workspace,
    Result,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Position},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState, Paragraph},
    Frame,
};

enum ActiveElement {
    ProgramsList,
    SearchBar,
}

pub enum Message {
    CreateCommand(NewCommand),
    DeleteCommand,
    Exit,
    SelectNextCommand,
    SelectPreviousCommand,
}

pub struct Model<'a> {
    active_element: ActiveElement,
    organizer: &'a mut OrganizerClient,
    search_bar: Input,
    selected_command_index: Option<usize>,
    state: State,
    workspace_index: usize,
}

pub struct ModelParameters<'a> {
    pub organizer: &'a mut OrganizerClient,
    pub workspace_index: usize,
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
    command_name: &'a str,
    programs: &'a [String],
    search_bar: &'a Input,
    selected_command_index: Option<usize>,
    workspace_name: &'a str,
}

impl ActiveElement {
    fn is_search_bar(&self) -> bool {
        matches!(self, ActiveElement::SearchBar)
    }

    fn toggle(&mut self) {
        *self = match *self {
            ActiveElement::ProgramsList => ActiveElement::SearchBar,
            ActiveElement::SearchBar => ActiveElement::ProgramsList,
        };
    }
}

impl<'a> Model<'a> {
    fn create_command(&mut self, parameters: NewCommand) -> Result<()> {
        let NewCommand { name, program } = parameters;

        self.organizer
            .create_command(self.workspace_index, name, program)?;
        self.selected_command_index = Some(self.programs()?.len() - 1);

        Ok(())
    }

    pub fn delete_command(&mut self) -> Result<()> {
        let Some(index) = self.selected_command_index else {
            return Ok(());
        };

        self.organizer.delete_command(self.workspace_index, index)?;
        self.selected_command_index = None;

        Ok(())
    }

    fn exit(&mut self) {
        self.state = State::Exited;
    }

    pub fn is_exited(&self) -> bool {
        matches!(self.state, State::Exited)
    }

    pub fn new(params: ModelParameters<'a>) -> Self {
        let ModelParameters {
            workspace_index,
            organizer,
        } = params;

        Self {
            active_element: ActiveElement::SearchBar,
            organizer,
            search_bar: Input::default(),
            selected_command_index: None,
            state: State::Running,
            workspace_index,
        }
    }

    fn programs(&self) -> Result<Vec<String>> {
        self.workspace().map(|workspace| {
            workspace
                .commands
                .iter()
                .map(|command| command.program.clone())
                .collect()
        })
    }

    fn select_next(&mut self) -> Result<()> {
        if self.workspace()?.commands.is_empty() {
            return Ok(());
        }

        let Some(index) = self.selected_command_index else {
            self.active_element.toggle();
            self.selected_command_index = Some(0);

            return Ok(());
        };

        if index == (self.workspace()?.commands.len() - 1) {
            self.selected_command_index = None;
            self.active_element.toggle();
        } else {
            self.selected_command_index = Some(index + 1);
        }

        Ok(())
    }

    fn select_prev(&mut self) -> Result<()> {
        if self.workspace()?.commands.is_empty() {
            return Ok(());
        }

        let Some(index) = self.selected_command_index else {
            self.selected_command_index = Some(self.workspace()?.commands.len() - 1);
            self.active_element.toggle();

            return Ok(());
        };

        if index == 0 {
            self.selected_command_index = None;
            self.active_element.toggle();
        } else {
            self.selected_command_index = Some(index - 1);
        }

        Ok(())
    }

    pub fn tableau(&mut self) -> Result<Option<TableauModel>> {
        let Some(nidex) = self.selected_command_index else {
            return Ok(None);
        };

        let command = self.organizer.get_command(self.workspace_index, nidex)?;
        let CommandExecutorOutput { stdout, stderr } = CommandExecutor::new(&command).execute()?;
        let model = TableauModel::new(TableauModelParameters {
            command,
            stdout,
            stderr,
        });

        Ok(Some(model))
    }

    pub fn update(&mut self, message: Message) -> Result<()> {
        match message {
            Message::SelectPreviousCommand => self.select_prev()?,
            Message::SelectNextCommand => self.select_next()?,
            Message::DeleteCommand => self.delete_command()?,
            Message::Exit => self.exit(),
            Message::CreateCommand(parameters) => self.create_command(parameters)?,
        }

        Ok(())
    }

    pub fn view(&self, frame: &mut Frame) {
        let workspace = self.organizer.get_workspace(self.workspace_index).unwrap();
        let programs: Vec<String> = workspace
            .commands
            .iter()
            .map(|command| command.program.clone())
            .collect();

        let command_name = if let Some(index) = self.selected_command_index {
            let command = &workspace.commands[index];
            &command.name
        } else {
            ""
        };

        let view = View {
            programs: &programs,
            selected_command_index: self.selected_command_index,
            workspace_name: &workspace.name,
            command_name,
            active_element: &self.active_element,
            search_bar: &self.search_bar,
        };

        view.render(frame);
    }

    fn workspace(&self) -> Result<Workspace> {
        self.organizer.get_workspace(self.workspace_index)
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

        let list = List::new(self.programs.to_vec())
            .highlight_style(Style::new().reversed())
            .block(
                Block::new()
                    .title(format!("{} commands", self.workspace_name))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::all()),
            );
        let mut state = ListState::default();

        state.select(self.selected_command_index);

        frame.render_stateful_widget(list, programs_list, &mut state);

        let paragraph = Paragraph::new(self.command_name).block(
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
