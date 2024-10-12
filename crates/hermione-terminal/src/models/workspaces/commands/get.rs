use crate::{
    breadcrumbs::Breadcrumbs,
    components, layouts, parameters, presenters,
    routes::{self, Route},
    tui, Message, Result,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Model {
    workspace: presenters::workspace::Presenter,
    command: presenters::command::Presenter,
    redirect: Option<Route>,
    command_palette: Option<components::command_palette::Component>,
}

pub struct ModelParameters {
    pub command: presenters::command::Presenter,
    pub workspace: presenters::workspace::Presenter,
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
        let [main_area, status_bar_area] = layouts::full_width::Layout::new().areas(frame.area());

        let [name_area, program_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Max(3), Constraint::Min(3)])
            .areas(main_area);

        let block = Block::default().borders(Borders::all()).title("Name");
        let paragraph = Paragraph::new(self.command.name.as_str()).block(block);
        frame.render_widget(paragraph, name_area);

        let block = Block::default().borders(Borders::all()).title("Program");
        let paragraph = Paragraph::new(self.command.program.as_str()).block(block);
        frame.render_widget(paragraph, program_area);

        let paragraph = Paragraph::new(self.breadcrumbs());
        frame.render_widget(paragraph, status_bar_area);

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
                    search_query: self.command.program.clone(),
                    page_number: 0,
                    page_size: 10,
                },
            ),
        ));

        self.redirect = Some(route)
    }

    fn breadcrumbs(&self) -> Breadcrumbs {
        Breadcrumbs::default()
            .add_segment("List workspaces")
            .add_segment(&self.workspace.name)
            .add_segment("List commands")
            .add_segment(&self.command.name)
    }

    pub fn new(parameters: ModelParameters) -> Result<Self> {
        let ModelParameters { command, workspace } = parameters;

        Ok(Self {
            command,
            workspace,
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
