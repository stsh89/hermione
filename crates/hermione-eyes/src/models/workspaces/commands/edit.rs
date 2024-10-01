use crate::{
    app::Message,
    helpers::{Input, InputParameters},
    parameters,
    presenters::command::Presenter,
    routes::{self, Route},
    tui, Result,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Model {
    command: Presenter,
    active_input: CommandProperty,
    name: Input,
    program: Input,
    redirect: Option<Route>,
}

pub struct ModelParameters {
    pub command: Presenter,
}

enum CommandProperty {
    Name,
    Program,
}

impl tui::Model for Model {
    type Message = Message;
    type Route = Route;

    fn handle_event(&self) -> Result<Option<Self::Message>> {
        tui::EventHandler::new(|key_event| key_event.try_into().ok()).handle_event()
    }

    fn redirect(&mut self) -> Option<Route> {
        self.redirect.take()
    }

    fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::Back => self.back(),
            Message::ToggleFocus => self.toggle_focus(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::Submit => self.submit(),
            Message::Action
            | Message::SelectNext
            | Message::SelectPrevious
            | Message::ToggleCommandPalette => {}
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        let [header, name, program] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(1),
                Constraint::Max(3),
                Constraint::Min(3),
            ])
            .areas(frame.area());

        let paragraph = Paragraph::new("Edit Command").alignment(Alignment::Center);
        frame.render_widget(paragraph, header);

        let block = Block::default().borders(Borders::all()).title("Name");
        let paragraph = Paragraph::new(self.name.value()).block(block);

        frame.render_widget(paragraph, name);

        if self.name.is_active() {
            frame.set_cursor_position(Position::new(
                name.x + self.name.character_index() as u16 + 1,
                name.y + 1,
            ));
        }
        let block = Block::default().borders(Borders::all()).title("Program");
        let paragraph = Paragraph::new(self.program.value()).block(block);

        frame.render_widget(paragraph, program);

        if self.program.is_active() {
            frame.set_cursor_position(Position::new(
                program.x + self.program.character_index() as u16 + 1,
                program.y + 1,
            ));
        }
    }
}

impl Model {
    fn back(&mut self) {
        let route = Route::Workspaces(routes::workspaces::Route::Commands(
            routes::workspaces::commands::Route::Get(
                parameters::workspaces::commands::get::Parameters {
                    workspace_id: self.command.workspace_id.clone(),
                    command_id: self.command.id.clone(),
                },
            ),
        ));

        self.redirect = Some(route);
    }

    fn delete_char(&mut self) {
        match self.active_input {
            CommandProperty::Name => self.name.delete_char(),
            CommandProperty::Program => self.program.delete_char(),
        }
    }

    fn delete_all_chars(&mut self) {
        match self.active_input {
            CommandProperty::Name => self.name.delete_all_chars(),
            CommandProperty::Program => self.program.delete_all_chars(),
        }
    }

    fn enter_char(&mut self, c: char) {
        match self.active_input {
            CommandProperty::Name => self.name.enter_char(c),
            CommandProperty::Program => self.program.enter_char(c),
        }
    }

    fn move_cursor_left(&mut self) {
        match self.active_input {
            CommandProperty::Name => self.name.move_cursor_left(),
            CommandProperty::Program => self.program.move_cursor_left(),
        }
    }

    fn move_cursor_right(&mut self) {
        match self.active_input {
            CommandProperty::Name => self.name.move_cursor_right(),
            CommandProperty::Program => self.program.move_cursor_right(),
        }
    }

    pub fn new(parameters: ModelParameters) -> Self {
        let ModelParameters { command } = parameters;

        Self {
            name: Input::new(InputParameters {
                value: command.name.clone(),
                is_active: true,
            }),
            redirect: None,
            active_input: CommandProperty::Name,
            program: Input::new(InputParameters {
                value: command.program.clone(),
                is_active: false,
            }),
            command,
        }
    }

    fn submit(&mut self) {
        let route = Route::Workspaces(routes::workspaces::Route::Commands(
            routes::workspaces::commands::Route::Update(
                parameters::workspaces::commands::update::Parameters {
                    name: self.name.value().to_string(),
                    program: self.program.value().to_string(),
                    workspace_id: self.command.workspace_id.clone(),
                    command_id: self.command.id.clone(),
                },
            ),
        ));

        self.redirect = Some(route);
    }

    fn toggle_focus(&mut self) {
        self.active_input = match self.active_input {
            CommandProperty::Name => CommandProperty::Program,
            CommandProperty::Program => CommandProperty::Name,
        };

        match self.active_input {
            CommandProperty::Name => {
                self.name.activate();
                self.program.deactivate();
            }
            CommandProperty::Program => {
                self.program.activate();
                self.name.deactivate();
            }
        }
    }
}
