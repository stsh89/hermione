use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Alignment, Constraint, Direction, Layout, Position},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{
    entities::{Command, Workspace},
    models::{
        command_palette::DELETE_COMMAND,
        handle_event, highlight_style,
        shared::{Input, InputParameters},
        Message,
    },
    router::{CommandPaletteParameters, GetWorkspaceParameters, ListWorkspacesParameters, Router},
    Result,
};

pub struct GetCommandModel {
    command: Command,
    is_running: bool,
    redirect: Option<Router>,
}

pub struct GetCommandModelParameters {
    pub command: Command,
}

impl GetCommandModel {
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn new(parameters: GetCommandModelParameters) -> Self {
        let GetCommandModelParameters { command } = parameters;

        Self {
            command,
            is_running: true,
            redirect: None,
        }
    }

    pub fn handle_event(&self) -> Result<Option<Message>> {
        handle_event(message)
    }

    pub fn redirect(&self) -> Option<&Router> {
        self.redirect.as_ref()
    }

    pub fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::Back => self.redirect_to_workspace(),
            Message::ActivateCommandPalette => self.redirect_to_command_palette(),
            _ => {}
        }

        Ok(None)
    }

    fn redirect_to_workspace(&mut self) {
        self.redirect = Some(Router::GetWorkspace(GetWorkspaceParameters {
            number: 0,
            commands_search_query: String::new(),
        }))
    }

    fn redirect_to_command_palette(&mut self) {
        self.redirect = Some(Router::CommandPalette(CommandPaletteParameters {
            actions: vec![DELETE_COMMAND.to_string()],
        }))
    }

    pub fn view(&mut self, frame: &mut Frame) {
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

fn message(key_event: KeyEvent) -> Option<Message> {
    let message = match key_event.code {
        KeyCode::Esc => Message::Back,
        KeyCode::Char('k') if key_event.modifiers == KeyModifiers::CONTROL => {
            Message::ActivateCommandPalette
        }
        _ => return None,
    };

    Some(message)
}
