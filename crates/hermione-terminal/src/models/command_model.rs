use super::{Command, Workspace};
use crate::{
    coordinator::CommandId,
    layouts::WideLayout,
    themes::Theme,
    tui::{EventHandler, Input, Model},
    widgets::FormField,
    CreateWorkspaceCommandParams, ExecuteProgramParams, ListWorkspaceCommandsParams, Message,
    Result, Route, UpdateWorkspaceCommandParams,
};
use ratatui::{
    layout::{Constraint, Direction, Position, Rect},
    style::Stylize,
    text::ToText,
    widgets::Paragraph,
    Frame,
};

pub struct CommandModel {
    active_input: InputName,
    id: Option<CommandId>,
    name: Input,
    program: Input,
    redirect: Option<Route>,
    theme: Theme,
    workspace: Workspace,
}

#[derive(Default, PartialEq)]
enum InputName {
    #[default]
    Name,
    Program,
}

impl Model for CommandModel {
    type Message = Message;
    type Route = Route;

    fn handle_event(&self) -> Result<Option<Self::Message>> {
        EventHandler::new(|key_event| key_event.try_into().ok()).handle_event()
    }

    fn redirect(&mut self) -> Option<Route> {
        self.redirect.take()
    }

    fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::Cancel => self.back(),
            Message::Tab => self.toggle_focus(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::Submit => self.submit(),
            Message::ExecuteCommand => self.execute_program(),
            Message::SelectNext | Message::SelectPrevious => {}
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        let [main_area, status_bar_area] = WideLayout::new().areas(frame.area());
        let input_areas = input_areas(main_area);

        self.render_name(frame, input_areas[0]);
        self.render_program(frame, input_areas[1]);
        self.render_status_bar(frame, status_bar_area);

        self.set_cursor_position(frame, input_areas[self.active_input.as_index()]);
    }
}

impl CommandModel {
    fn active_input(&self) -> &Input {
        match self.active_input {
            InputName::Name => &self.name,
            InputName::Program => &self.program,
        }
    }

    fn active_input_mut(&mut self) -> &mut Input {
        match self.active_input {
            InputName::Name => &mut self.name,
            InputName::Program => &mut self.program,
        }
    }

    fn back(&mut self) {
        self.set_redirect(
            ListWorkspaceCommandsParams {
                search_query: String::new(),
                page_number: None,
                page_size: None,
                workspace_id: self.workspace.id,
                powershell_no_exit: false,
            }
            .into(),
        );
    }

    fn create_command_parameters(&self) -> CreateWorkspaceCommandParams {
        CreateWorkspaceCommandParams {
            name: self.name.value().to_string(),
            program: self.program.value().to_string(),
            workspace_id: self.workspace.id,
        }
    }

    fn delete_char(&mut self) {
        self.active_input_mut().delete_char();
    }

    fn delete_all_chars(&mut self) {
        self.active_input_mut().delete_all_chars();
    }

    fn enter_char(&mut self, c: char) {
        self.active_input_mut().enter_char(c);
    }

    fn execute_program(&mut self) {
        self.set_redirect(
            ExecuteProgramParams {
                workspace_id: self.workspace.id,
                program: self.program.value().to_string(),
            }
            .into(),
        )
    }

    fn move_cursor_left(&mut self) {
        self.active_input_mut().move_cursor_left();
    }

    fn move_cursor_right(&mut self) {
        self.active_input_mut().move_cursor_right();
    }

    pub fn command(self, command: Command) -> Self {
        let Command {
            workspace_id: _,
            id,
            name,
            program,
        } = command;

        Self {
            name: Input::new(name),
            program: Input::new(program),
            id: Some(id),
            ..self
        }
    }

    pub fn new(workspace: Workspace) -> Self {
        Self {
            workspace,
            redirect: None,
            theme: Default::default(),
            active_input: Default::default(),
            name: Default::default(),
            program: Default::default(),
            id: None,
        }
    }

    fn render_program(&self, frame: &mut Frame, area: Rect) {
        let mut field = FormField::default()
            .name(InputName::Program.as_str())
            .value(self.program.value())
            .set_background_color(self.theme.background_color)
            .set_foreground_color(self.theme.foreground_color);

        if InputName::Program == self.active_input {
            field = field.set_foreground_color(self.theme.input_color);
        }

        frame.render_widget(field, area);
    }

    fn render_name(&self, frame: &mut Frame, area: Rect) {
        let mut field = FormField::default()
            .name(InputName::Name.as_str())
            .value(self.name.value())
            .set_background_color(self.theme.background_color)
            .set_foreground_color(self.theme.foreground_color);

        if InputName::Name == self.active_input {
            field = field.set_foreground_color(self.theme.input_color);
        }

        frame.render_widget(field, area);
    }

    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let screen = if self.id.is_some() {
            "Edit command"
        } else {
            "New command"
        };

        let value = serde_json::json!({
            "screen": screen,
            "workspace": self.workspace.name
        });

        let paragraph = Paragraph::new(value.to_text())
            .bg(self.theme.status_bar_background_color)
            .fg(self.theme.status_bar_foreground_color);

        frame.render_widget(paragraph, area);
    }

    fn set_cursor_position(&self, frame: &mut Frame, area: Rect) {
        frame.set_cursor_position(Position::new(
            area.x + self.active_input().character_index() as u16 + 1,
            area.y + 1,
        ));
    }

    fn set_redirect(&mut self, route: Route) {
        self.redirect = Some(route);
    }

    fn submit(&mut self) {
        let route = if let Some(id) = self.id {
            self.update_command_parameters(id).into()
        } else {
            self.create_command_parameters().into()
        };

        self.set_redirect(route);
    }

    pub fn theme(self, theme: Theme) -> Self {
        Self { theme, ..self }
    }

    fn toggle_focus(&mut self) {
        self.active_input = self.active_input.next();
    }

    fn update_command_parameters(&self, id: CommandId) -> UpdateWorkspaceCommandParams {
        UpdateWorkspaceCommandParams {
            name: self.name.value().to_string(),
            program: self.program.value().to_string(),
            workspace_id: self.workspace.id,
            command_id: id,
        }
    }
}

impl InputName {
    fn as_index(&self) -> usize {
        match self {
            InputName::Name => 0,
            InputName::Program => 1,
        }
    }

    fn as_str(&self) -> &str {
        match self {
            InputName::Name => "Name",
            InputName::Program => "Program",
        }
    }

    fn next(&self) -> Self {
        match self {
            InputName::Name => InputName::Program,
            InputName::Program => InputName::Name,
        }
    }
}

fn input_areas(main_area: Rect) -> [Rect; 2] {
    ratatui::layout::Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(3), Constraint::Min(3)])
        .areas(main_area)
}
