use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    entities::Command,
    models::{command_palette::DELETE_COMMAND, Message, Model},
    router::{CommandPaletteParameters, GetWorkspaceParameters, Router},
    Result,
};

pub struct GetCommandModel {
    command: Command,
    redirect: Option<Router>,
}

pub struct GetCommandModelParameters {
    pub command: Command,
}

impl Model for GetCommandModel {
    fn redirect(&self) -> Option<&Router> {
        self.redirect.as_ref()
    }

    fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::Back => self.back(),
            Message::ActivateCommandPalette => self.activate_command_palette(),
            _ => {}
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        let [header, program] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Max(1), Constraint::Min(3)])
            .areas(frame.area());

        let paragraph = Paragraph::new(self.command.name.as_str()).alignment(Alignment::Center);
        frame.render_widget(paragraph, header);

        let block = Block::default().borders(Borders::all()).title("Program");
        let paragraph = Paragraph::new(self.command.program.as_str()).block(block);

        frame.render_widget(paragraph, program);
    }
}

impl GetCommandModel {
    fn activate_command_palette(&mut self) {
        let route = Router::CommandPalette(CommandPaletteParameters {
            actions: vec![DELETE_COMMAND.to_string()],
        });

        self.redirect = Some(route)
    }

    fn back(&mut self) {
        let route = Router::GetWorkspace(GetWorkspaceParameters {
            number: 0,
            commands_search_query: String::new(),
        });

        self.redirect = Some(route)
    }

    pub fn new(parameters: GetCommandModelParameters) -> Self {
        let GetCommandModelParameters { command } = parameters;

        Self {
            command,
            redirect: None,
        }
    }
}
