use std::time::Duration;

use projection::{Projection, Workspace};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Flex, Layout, Position},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState, Paragraph},
    Frame,
};

pub struct Lens {
    projection: Projection,
    state: State,
    context: Context,
}

enum State {
    Open,
    Closed,
}

enum Context {
    Workspaces(WorkspacesContext),
    Workspace(WorkspaceContext),
    WorkspaceForm(WorkspaceFormContext),
}

impl Context {
    fn handle_key(&self, key_code: KeyCode) -> Option<Message> {
        match &self {
            Self::Workspaces(_) => match key_code {
                KeyCode::Char('q') => Some(Message::CloseLens),
                KeyCode::Esc => Some(Message::CloseLens),
                KeyCode::Up => Some(Message::SelectPreviousWorkspace),
                KeyCode::Down => Some(Message::SelectNextWorkspace),
                KeyCode::Enter => Some(Message::EnterWorkspace),
                KeyCode::Char('d') => Some(Message::DeleteWorkspace),
                KeyCode::Char('n') => Some(Message::EnterWorkspaceForm),
                _ => None,
            },
            Self::Workspace(_) => match key_code {
                KeyCode::Char('q') => Some(Message::CloseLens),
                KeyCode::Esc => Some(Message::ExitWorkspace),
                KeyCode::Up => Some(Message::SelectPreviousCommand),
                KeyCode::Down => Some(Message::SelectNextCommand),
                _ => None,
            },
            Self::WorkspaceForm(_) => match key_code {
                KeyCode::Esc => Some(Message::ExitWorkspaceForm),
                KeyCode::Enter => Some(Message::CreateWorkspace),
                KeyCode::Char(to_insert) => Some(Message::WorkspaceFormAddChar(to_insert)),
                KeyCode::Backspace => Some(Message::WorkspaceFormNameDeleteChar),
                KeyCode::Left => Some(Message::WorkspaceFormMoveCusorLeft),
                KeyCode::Right => Some(Message::WorkspaceFormMoveCusorRight),
                _ => None,
            },
        }
    }

    fn view(&self, frame: &mut Frame) {
        match &self {
            Self::Workspaces(inner) => inner.render(frame),
            Self::Workspace(inner) => inner.render(frame),
            Self::WorkspaceForm(inner) => inner.render(frame),
        }
    }
}

struct WorkspaceFormContext {
    /// Current value of the input box
    value: String,
    /// Position of cursor in the editor area.
    character_index: usize,
}

impl WorkspaceFormContext {
    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.value.insert(index, new_char);
        self.move_cursor_right();
    }

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

    fn delete_char(&mut self) {
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

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.value.chars().count())
    }

    fn render(&self, frame: &mut Frame) {
        let layout =
            Layout::new(Direction::Vertical, vec![Constraint::Percentage(100)]).flex(Flex::Start);

        let [top] = layout.areas(frame.area());

        let paragraph = Paragraph::new(self.value.as_str()).block(
            Block::new()
                .title("Enter workspace name")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );

        frame.render_widget(paragraph, top);

        frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position is can be controlled via the left and right arrow key
            top.x + self.character_index as u16 + 1,
            // Move one line down, from the border to the input line
            top.y + 1,
        ));
    }
}

struct WorkspacesContext {
    selected_workspace_index: Option<usize>,
    workspaces: Vec<String>,
    commands: Vec<String>,
}

struct WorkspaceContext {
    workspace_index: usize,
    selected_command_index: Option<usize>,
    commands: Vec<String>,
    selected_command_name: String,
    workspace_name: String,
}

impl WorkspacesContext {
    fn new(workspaces: Vec<String>) -> Self {
        Self {
            selected_workspace_index: None,
            commands: vec![],
            workspaces,
        }
    }

    fn render(&self, frame: &mut Frame) {
        let layout = Layout::new(
            Direction::Horizontal,
            vec![Constraint::Percentage(25), Constraint::Percentage(75)],
        )
        .flex(Flex::Start);
        let [left, right] = layout.areas(frame.area());

        let list = List::new(self.workspaces.iter().map(|w| w.as_str()))
            .highlight_style(Style::new().reversed())
            .block(
                Block::new()
                    .title("Workspaces")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::all()),
            );
        let mut state = ListState::default();

        state.select(self.selected_workspace_index);

        frame.render_stateful_widget(list, left, &mut state);

        let list = List::new(self.commands.iter().map(|c| c.as_str())).block(
            Block::new()
                .title("Commands")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(list, right)
    }
}

impl WorkspaceContext {
    fn render(&self, frame: &mut Frame) {
        let layout = Layout::new(
            Direction::Vertical,
            vec![Constraint::Percentage(100), Constraint::Min(3)],
        )
        .flex(Flex::Start);
        let [top, bottom] = layout.areas(frame.area());

        let list = List::new(self.commands.iter().map(|c| c.as_str()))
            .highlight_style(Style::new().reversed())
            .block(
                Block::new()
                    .title(format!("{} commands", self.workspace_name))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::all()),
            );
        let mut state = ListState::default();

        state.select(self.selected_command_index);

        frame.render_stateful_widget(list, top, &mut state);

        let paragraph = Paragraph::new(self.selected_command_name.as_str()).block(
            Block::new()
                .title("Command name")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(paragraph, bottom)
    }
}

pub enum Message {
    CloseLens,
    CreateWorkspace,
    DeleteWorkspace,
    EnterWorkspace,
    EnterWorkspaceForm,
    ExitWorkspace,
    ExitWorkspaceForm,
    SelectNextCommand,
    SelectNextWorkspace,
    SelectPreviousCommand,
    SelectPreviousWorkspace,
    WorkspaceFormAddChar(char),
    WorkspaceFormMoveCusorLeft,
    WorkspaceFormMoveCusorRight,
    WorkspaceFormNameDeleteChar,
}

impl Lens {
    pub fn is_closed(&self) -> bool {
        matches!(self.state, State::Closed)
    }

    fn close(&mut self) {
        self.state = State::Closed;
    }

    pub fn open(projection: Projection) -> Self {
        let workspaces = projection
            .workspaces()
            .iter()
            .map(|w| w.name().to_string())
            .collect();

        Self {
            projection,
            state: State::Open,
            context: Context::Workspaces(WorkspacesContext::new(workspaces)),
        }
    }

    fn workspace_names(&self) -> Vec<String> {
        self.projection
            .workspaces()
            .iter()
            .map(|w| w.name().to_string())
            .collect()
    }

    fn workspace_commands(&self, workspace_index: usize) -> Vec<String> {
        let Some(workspace) = self.projection.workspaces().get(workspace_index) else {
            return vec![];
        };

        workspace
            .instructions()
            .iter()
            .map(|i| i.directive().to_string())
            .collect()
    }

    pub fn view(&self, frame: &mut Frame) {
        self.context.view(frame);
    }

    pub fn handle_event(&self) -> std::io::Result<Option<Message>> {
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let message = self.context.handle_key(key.code);

                    return Ok(message);
                }
            }
        }

        Ok(None)
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::CloseLens => self.close(),

            Message::SelectNextWorkspace => self.select_next_workspace(),

            Message::SelectPreviousWorkspace => self.select_previous_workspace(),

            Message::EnterWorkspace => self.enter_workspace(),

            Message::ExitWorkspace => self.exit_workspace(),

            Message::SelectNextCommand => self.select_next_command(),

            Message::SelectPreviousCommand => self.select_previous_command(),

            Message::DeleteWorkspace => self.delete_workspace(),

            Message::ExitWorkspaceForm => self.exit_workspace_form(),

            Message::CreateWorkspace => self.create_workspace(),

            Message::WorkspaceFormAddChar(char) => self.workspace_form_add_char(char),

            Message::WorkspaceFormNameDeleteChar => self.workspace_form_delete_char(),

            Message::WorkspaceFormMoveCusorLeft => self.workspace_form_move_cursor_left(),

            Message::WorkspaceFormMoveCusorRight => self.workspace_form_move_cursor_right(),

            Message::EnterWorkspaceForm => self.enter_workspace_form(),
        }
    }

    fn enter_workspace_form(&mut self) {
        self.context = Context::WorkspaceForm(WorkspaceFormContext {
            value: "".to_string(),
            character_index: 0,
        });
    }

    fn workspace_form_move_cursor_left(&mut self) {
        let Context::WorkspaceForm(ref mut context) = self.context else {
            return;
        };

        context.move_cursor_left();
    }

    fn workspace_form_move_cursor_right(&mut self) {
        let Context::WorkspaceForm(ref mut context) = self.context else {
            return;
        };

        context.move_cursor_right();
    }

    fn create_workspace(&mut self) {
        let Context::WorkspaceForm(ref mut context) = self.context else {
            return;
        };

        let workspace = Workspace::new(context.value.clone());

        self.projection.add_workspace(workspace);
        self.context = Context::Workspaces(WorkspacesContext::new(self.workspace_names()));
    }

    fn workspace_form_add_char(&mut self, char: char) {
        let Context::WorkspaceForm(ref mut context) = self.context else {
            return;
        };

        context.enter_char(char);
    }

    fn workspace_form_delete_char(&mut self) {
        let Context::WorkspaceForm(ref mut context) = self.context else {
            return;
        };

        context.delete_char();
    }

    fn delete_workspace(&mut self) {
        let Context::Workspaces(context) = &self.context else {
            return;
        };

        let Some(index) = context.selected_workspace_index else {
            return;
        };

        self.projection.remove_workspace(index);
        self.context = Context::Workspaces(WorkspacesContext::new(self.workspace_names()));
    }

    fn enter_workspace(&mut self) {
        let Context::Workspaces(context) = &self.context else {
            return;
        };

        let Some(workspace_index) = context.selected_workspace_index else {
            return;
        };

        self.context = Context::Workspace(WorkspaceContext {
            workspace_index,
            commands: self.workspace_commands(workspace_index),
            selected_command_index: None,
            selected_command_name: "".to_string(),
            workspace_name: self.workspace_name(workspace_index),
        });
    }

    fn workspace_name(&self, workspace_index: usize) -> String {
        let Some(workspace) = self.projection.workspaces().get(workspace_index) else {
            return "".to_string();
        };

        workspace.name().to_string()
    }

    fn exit_workspace(&mut self) {
        self.context = Context::Workspaces(WorkspacesContext::new(self.workspace_names()));
    }

    fn exit_workspace_form(&mut self) {
        self.context = Context::Workspaces(WorkspacesContext::new(self.workspace_names()));
    }

    fn select_next_workspace(&mut self) {
        let Context::Workspaces(context) = &self.context else {
            return;
        };

        if context.workspaces.is_empty() {
            return;
        }

        let mut new_index = 0;

        if let Some(index) = context.selected_workspace_index {
            if index < (context.workspaces.len() - 1) {
                new_index = index + 1;
            }
        }

        self.context = Context::Workspaces(WorkspacesContext {
            selected_workspace_index: Some(new_index),
            workspaces: self.workspace_names(),
            commands: self.workspace_commands(new_index),
        });
    }

    fn select_next_command(&mut self) {
        let Context::Workspace(context) = &self.context else {
            return;
        };

        if context.commands.is_empty() {
            return;
        }

        let mut new_index = 0;

        if let Some(index) = context.selected_command_index {
            if index < (context.commands.len() - 1) {
                new_index = index + 1;
            }
        }

        self.context = Context::Workspace(WorkspaceContext {
            workspace_index: context.workspace_index,
            selected_command_index: Some(new_index),
            commands: context.commands.clone(),
            selected_command_name: self.command_name(context.workspace_index, new_index),
            workspace_name: context.workspace_name.clone(),
        });
    }

    fn select_previous_command(&mut self) {
        let Context::Workspace(context) = &self.context else {
            return;
        };

        if context.commands.is_empty() {
            return;
        }

        let mut new_index = context.commands.len() - 1;

        if let Some(index) = context.selected_command_index {
            if index > 0 {
                new_index = index - 1;
            }
        }

        self.context = Context::Workspace(WorkspaceContext {
            workspace_index: context.workspace_index,
            selected_command_index: Some(new_index),
            commands: context.commands.clone(),
            selected_command_name: self.command_name(context.workspace_index, new_index),
            workspace_name: context.workspace_name.clone(),
        });
    }

    fn command_name(&self, workspace_index: usize, command_index: usize) -> String {
        let Some(workspace) = self.projection.workspaces().get(workspace_index) else {
            return "".to_string();
        };

        let Some(command) = workspace.instructions().get(command_index) else {
            return "".to_string();
        };

        command.name().to_string()
    }

    fn select_previous_workspace(&mut self) {
        let Context::Workspaces(context) = &self.context else {
            return;
        };

        if context.workspaces.is_empty() {
            return;
        }

        let mut new_index = context.workspaces.len() - 1;

        if let Some(index) = context.selected_workspace_index {
            if index > 0 {
                new_index = index - 1;
            }
        }

        self.context = Context::Workspaces(WorkspacesContext {
            selected_workspace_index: Some(new_index),
            workspaces: self.workspace_names(),
            commands: self.workspace_commands(new_index),
        });
    }
}
