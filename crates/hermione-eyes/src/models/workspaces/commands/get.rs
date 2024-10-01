use crate::{
    app::Message,
    helpers::{CommandPalette, CommandPaletteAction, CommandPaletteParameters},
    parameters,
    presenters::command::Presenter,
    routes::{self, Route},
    tui, Result,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Model {
    command: Presenter,
    redirect: Option<Route>,
    command_palette: CommandPalette,
}

pub struct ModelParameters {
    pub command: Presenter,
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
                self.redirect = Some(Route::Workspaces(routes::workspaces::Route::Commands(
                    routes::workspaces::commands::Route::Delete(
                        parameters::workspaces::commands::delete::Parameters {
                            workspace_id: self.command.workspace_id.clone(),
                            command_id: self.command.id.clone(),
                        },
                    ),
                )))
            }
            CPA::EditCommand => {
                self.redirect = Some(Route::Workspaces(routes::workspaces::Route::Commands(
                    routes::workspaces::commands::Route::Edit(
                        parameters::workspaces::commands::edit::Parameters {
                            workspace_id: self.command.workspace_id.clone(),
                            command_id: self.command.id.clone(),
                        },
                    ),
                )))
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
        let route = Route::Workspaces(routes::workspaces::Route::Commands(
            routes::workspaces::commands::Route::List(
                parameters::workspaces::commands::list::Parameters {
                    workspace_id: self.command.workspace_id.clone(),
                    search_query: Some(self.command.program.clone()),
                },
            ),
        ));

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
