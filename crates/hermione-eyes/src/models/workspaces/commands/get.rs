use crate::{
    app::{Hook, Message},
    helpers::{CommandPalette, CommandPaletteAction, CommandPaletteParameters},
    presenters::command::Presenter,
    routes::{self, Router},
    Result,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Model {
    command: Presenter,
    redirect: Option<Router>,
    command_palette: CommandPalette,
}

pub struct ModelParameters {
    pub command: Presenter,
}

impl Hook for Model {
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
            Message::Action
            | Message::DeleteAllChars
            | Message::DeleteChar
            | Message::EnterChar(_)
            | Message::MoveCusorLeft
            | Message::MoveCusorRight
            | Message::ToggleFocus => {}
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

impl Model {
    fn handle_command_palette_action(&mut self) {
        use CommandPaletteAction as CPA;

        let Some(action) = self.command_palette.action() else {
            return;
        };

        match action {
            CPA::DeleteCommand => {
                self.redirect = Some(
                    routes::workspaces::commands::delete::Parameters {
                        workspace_id: self.command.workspace_id.clone(),
                        command_id: self.command.id.clone(),
                    }
                    .into(),
                )
            }
            CPA::EditCommand => {
                self.redirect = Some(
                    routes::workspaces::commands::edit::Parameters {
                        workspace_id: self.command.workspace_id.clone(),
                        command_id: self.command.id.clone(),
                    }
                    .into(),
                )
            }
            CPA::CopyToClipboard
            | CPA::DeleteWorkspace
            | CPA::EditWorkspace
            | CPA::ListWorkspaces
            | CPA::NewCommand
            | CPA::NewWorkspace
            | CPA::SetPowershellNoExit
            | CPA::StartWindowsTerminal
            | CPA::UnsetPowerShellNoExit => {}
        }
    }

    fn toggle_command_palette(&mut self) {
        self.command_palette.toggle();
    }

    fn back(&mut self) {
        let route = routes::workspaces::commands::list::Parameters {
            search_query: String::new(),
            workspace_id: self.command.workspace_id.clone(),
        }
        .into();

        self.redirect = Some(route)
    }

    pub fn new(parameters: ModelParameters) -> Result<Self> {
        use CommandPaletteAction as CPA;

        let ModelParameters { command } = parameters;

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
