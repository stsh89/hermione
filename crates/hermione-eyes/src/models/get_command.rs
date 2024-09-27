use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    models::{
        helpers::{CommandPalette, CommandPaletteParameters},
        Message, Model,
    },
    router::{DeleteCommandParameters, EditCommandParameters, GetWorkspaceParameters, Router},
    types::{Command, Result},
};

pub struct GetCommandModel {
    command: Command,
    redirect: Option<Router>,
    command_palette: CommandPalette,
}

pub struct GetCommandModelParameters {
    pub command: Command,
}

impl Model for GetCommandModel {
    fn redirect(&mut self) -> Option<Router> {
        self.redirect.take()
    }

    fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::Back => self.back(),
            Message::ToggleCommandPalette => self.toggle_command_palette(),
            Message::Submit => self.submit(),
            Message::SelectNext => self.select_next(),
            Message::SelectPrevious => self.select_previous(),
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

        if self.command_palette.is_active() {
            self.command_palette.render(frame, frame.area());
        }
    }
}

impl GetCommandModel {
    fn handle_command_palette_action(&mut self) {
        use crate::models::helpers::CommandPaletteAction as CPA;

        let Some(action) = self.command_palette.action() else {
            return;
        };

        match action {
            CPA::DeleteCommand => {
                self.redirect = Some(Router::DeleteCommand(DeleteCommandParameters {
                    workspace_id: self.command.workspace_id.clone(),
                    command_id: self.command.id().to_string(),
                }))
            }
            CPA::EditCommand => {
                self.redirect = Some(Router::EditCommand(EditCommandParameters {
                    workspace_id: self.command.workspace_id.clone(),
                    command_id: self.command.id().to_string(),
                }))
            }
            _ => {}
        }
    }

    fn toggle_command_palette(&mut self) {
        self.command_palette.toggle();
    }

    fn back(&mut self) {
        let route = Router::GetWorkspace(GetWorkspaceParameters {
            commands_search_query: String::new(),
            id: self.command.workspace_id.clone(),
        });

        self.redirect = Some(route)
    }

    pub fn new(parameters: GetCommandModelParameters) -> Result<Self> {
        use crate::models::helpers::CommandPaletteAction as CPA;

        let GetCommandModelParameters { command } = parameters;

        Ok(Self {
            command,
            redirect: None,
            command_palette: CommandPalette::new(CommandPaletteParameters {
                actions: vec![CPA::DeleteCommand, CPA::EditCommand],
            })?,
        })
    }

    fn select_next(&mut self) {
        if self.command_palette.is_active() {
            self.command_palette.select_next();
        }
    }

    fn select_previous(&mut self) {
        if self.command_palette.is_active() {
            self.command_palette.select_previous();
        }
    }

    fn submit(&mut self) {
        if self.command_palette.is_active() {
            self.handle_command_palette_action();
        }
    }
}
