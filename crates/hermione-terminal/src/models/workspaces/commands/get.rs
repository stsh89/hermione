use crate::{
    components, parameters,
    presenters::command::Presenter,
    routes::{self, Route},
    tui, Message, Result,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Model {
    command: Presenter,
    redirect: Option<Route>,
    command_palette: Option<components::command_palette::Component>,
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
            Message::ActivateCommandPalette => self.activate_command_palette()?,
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

        if let Some(popup) = self.command_palette.as_mut() {
            popup.render(frame, frame.area());
        }
    }
}

impl Model {
    fn activate_command_palette(&mut self) -> Result<()> {
        use components::command_palette::Action;

        self.command_palette = Some(components::command_palette::Component::new(
            components::command_palette::ComponentParameters {
                actions: vec![Action::DeleteCommand, Action::EditCommand],
            },
        )?);

        Ok(())
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
        let ModelParameters { command } = parameters;

        Ok(Self {
            command,
            redirect: None,
            command_palette: None,
        })
    }

    fn select_next(&mut self) {
        if let Some(command_palette) = self.command_palette.as_mut() {
            command_palette.select_next();
        }
    }

    fn select_previous(&mut self) {
        if let Some(command_palette) = self.command_palette.as_mut() {
            command_palette.select_previous();
        }
    }

    fn submit(&mut self) {
        let Some(action) = self
            .command_palette
            .as_ref()
            .and_then(|command_palette| command_palette.action())
        else {
            return;
        };

        use components::command_palette::Action;
        match action {
            Action::DeleteCommand => {
                self.redirect = Some(Route::Workspaces(routes::workspaces::Route::Commands(
                    routes::workspaces::commands::Route::Delete(
                        parameters::workspaces::commands::delete::Parameters {
                            workspace_id: self.command.workspace_id.clone(),
                            command_id: self.command.id.clone(),
                        },
                    ),
                )))
            }
            Action::EditCommand => {
                self.redirect = Some(Route::Workspaces(routes::workspaces::Route::Commands(
                    routes::workspaces::commands::Route::Edit(
                        parameters::workspaces::commands::edit::Parameters {
                            workspace_id: self.command.workspace_id.clone(),
                            command_id: self.command.id.clone(),
                        },
                    ),
                )))
            }
            Action::CopyToClipboard
            | Action::DeleteWorkspace
            | Action::EditWorkspace
            | Action::ListWorkspaces
            | Action::NewCommand
            | Action::NewWorkspace
            | Action::SetPowershellNoExit
            | Action::StartWindowsTerminal
            | Action::UnsetPowerShellNoExit => {}
        }
    }
}
