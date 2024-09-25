use crate::{
    models::{helpers::Input, Message, Model, Router},
    router::{CreateWorkspaceParameters, ListWorkspacesParameters},
    Result,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct NewWorkspaceModel {
    name: Input,
    redirect: Option<Router>,
}

impl Model for NewWorkspaceModel {
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
            Message::Submit => self.submit(),
            _ => {}
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        let [header, name] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Max(1), Constraint::Min(3)])
            .areas(frame.area());

        let paragraph = Paragraph::new("New workspace").alignment(Alignment::Center);
        frame.render_widget(paragraph, header);

        let block = Block::default().borders(Borders::all()).title("Name");
        let paragraph = Paragraph::new(self.name.value()).block(block);

        frame.render_widget(paragraph, name);
        frame.set_cursor_position(Position::new(
            name.x + self.name.character_index() as u16 + 1,
            name.y + 1,
        ));
    }
}

impl NewWorkspaceModel {
    fn back(&mut self) {
        let route = Router::ListWorkspaces(ListWorkspacesParameters::default());

        self.redirect = Some(route);
    }

    fn delete_char(&mut self) {
        self.name.delete_char();
    }

    fn delete_all_chars(&mut self) {
        self.name.delete_all_chars();
    }

    fn enter_char(&mut self, c: char) {
        self.name.enter_char(c);
    }

    fn move_cursor_left(&mut self) {
        self.name.move_cursor_left();
    }

    fn move_cursor_right(&mut self) {
        self.name.move_cursor_right();
    }

    pub fn new() -> Self {
        Self {
            name: Input::active(),
            redirect: None,
        }
    }

    fn submit(&mut self) {
        let route = Router::CreateWorkspace(CreateWorkspaceParameters {
            name: self.name.value().to_string(),
        });

        self.redirect = Some(route);
    }
}
