use crate::{
    breadcrumbs::Breadcrumbs,
    components, layouts, parameters,
    presenters::workspace::Presenter,
    routes::{self, Route},
    tui::{self, Input},
    widgets, Message, Result,
};
use ratatui::{
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Model {
    is_running: bool,
    redirect: Option<Route>,
    search: Input,
    workspaces_state: widgets::list::State,
    workspaces: Vec<Presenter>,
    active_popup: Option<ActivePopup>,
    page_number: u32,
    page_size: u32,
}

pub struct ModelParameters {
    pub workspaces: Vec<Presenter>,
    pub search_query: String,
    pub page_number: u32,
    pub page_size: u32,
}

enum ActivePopup {
    CommandPalette(components::command_palette::Component),
    ExitConfirmation(components::confirmation::Component),
}

impl ActivePopup {
    fn exit_confirmation() -> Self {
        let confirmation = components::confirmation::Component::new(
            components::confirmation::ComponentParameters {
                message: "Press \"Enter\" to exit...".into(),
            },
        );

        Self::ExitConfirmation(confirmation)
    }

    fn command_palette() -> Result<Self> {
        use components::command_palette::Action;

        let command_palette = components::command_palette::Component::new(
            components::command_palette::ComponentParameters {
                actions: vec![Action::NewWorkspace],
            },
        )?;

        Ok(Self::CommandPalette(command_palette))
    }
}

impl tui::Model for Model {
    type Message = Message;
    type Route = Route;

    fn handle_event(&self) -> Result<Option<Self::Message>> {
        tui::EventHandler::new(|key_event| key_event.try_into().ok()).handle_event()
    }

    fn is_running(&self) -> bool {
        self.is_running
    }

    fn redirect(&mut self) -> Option<Route> {
        self.redirect.take()
    }

    fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::ActivateCommandPalette => self.activate_command_palette()?,
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::Back => self.back(),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::SelectNext => self.select_next(),
            Message::SelectPrevious => self.select_previous(),
            Message::Submit => self.submit(),
            Message::Action | Message::ToggleFocus => {}
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        let [main_area, status_bar_area] = layouts::full_width::Layout::new().areas(frame.area());
        let [search_area, list_area] = layouts::search_list::Layout::new().areas(main_area);

        let block = Block::default().borders(Borders::ALL).title("Search");
        let paragraph = Paragraph::new(self.search.value()).block(block);
        self.search.render(frame, search_area, paragraph);

        let block = Block::default().borders(Borders::all());
        let list = widgets::list::Widget::new(&self.workspaces).block(block);

        frame.render_stateful_widget(list, list_area, &mut self.workspaces_state);

        let paragraph = Paragraph::new(self.breadcrumbs());
        frame.render_widget(paragraph, status_bar_area);

        let Some(popup) = self.active_popup.as_mut() else {
            return;
        };

        match popup {
            ActivePopup::CommandPalette(popup) => popup.render(frame, frame.area()),
            ActivePopup::ExitConfirmation(popup) => popup.render(frame, frame.area()),
        }
    }
}

impl Model {
    fn activate_command_palette(&mut self) -> Result<()> {
        self.active_popup = Some(ActivePopup::command_palette()?);

        Ok(())
    }

    fn back(&mut self) {
        if self.active_popup.is_some() {
            self.active_popup = None;
        } else {
            self.active_popup = Some(ActivePopup::exit_confirmation());
        }
    }

    fn breadcrumbs(&self) -> Breadcrumbs {
        let breadcrumbs =
            Breadcrumbs::default().add_segment(format!("List workspaces ({})", self.page_number));

        if let Some(workspace) = self.workspace() {
            breadcrumbs.add_segment(&workspace.name)
        } else {
            breadcrumbs
        }
    }

    fn exit(&mut self) {
        self.is_running = false;
    }

    pub fn new(parameters: ModelParameters) -> Result<Self> {
        let ModelParameters {
            workspaces,
            search_query,
            page_number,
            page_size,
        } = parameters;

        let mut model = Self {
            workspaces,
            redirect: None,
            workspaces_state: widgets::list::State::default(),
            search: Input::new(search_query),
            is_running: true,
            active_popup: None,
            page_number,
            page_size,
        };

        if !model.workspaces.is_empty() {
            model.workspaces_state.select_first();
        }

        Ok(model)
    }

    fn submit(&mut self) {
        if let Some(active_popup) = &mut self.active_popup {
            match active_popup {
                ActivePopup::CommandPalette(popup) => {
                    let Some(action) = popup.action() else {
                        return;
                    };

                    use components::command_palette::Action;

                    if let Action::NewWorkspace = action {
                        self.redirect = Some(Route::Workspaces(routes::workspaces::Route::New))
                    }
                }
                ActivePopup::ExitConfirmation(_popup) => self.exit(),
            }

            self.active_popup = None;

            return;
        }

        let Some(workspace) = self.workspace() else {
            return;
        };

        let route = Route::Workspaces(routes::workspaces::Route::Commands(
            routes::workspaces::commands::Route::List(
                parameters::workspaces::commands::list::Parameters {
                    workspace_id: workspace.id.clone(),
                    search_query: "".into(),
                    page_number: 0,
                    page_size: parameters::workspaces::commands::list::PAGE_SIZE,
                },
            ),
        ));

        self.redirect = Some(route);
    }

    fn select_next(&mut self) {
        if let Some(popup) = self.active_popup.as_mut() {
            match popup {
                ActivePopup::CommandPalette(popup) => popup.select_next(),
                ActivePopup::ExitConfirmation(_popup) => {}
            };
        } else {
            let Some(index) = self.workspaces_state.selected() else {
                return;
            };

            if index == self.workspaces.len() - 1 {
                if self.workspaces.len() < self.page_size as usize {
                    return;
                }

                self.redirect = Some(Route::Workspaces(routes::workspaces::Route::List(
                    parameters::workspaces::list::Parameters {
                        search_query: self.search_query(),
                        page_number: self.page_number + 1,
                        page_size: self.page_size,
                    },
                )));

                return;
            }

            self.workspaces_state.select_next();
        }
    }

    fn select_previous(&mut self) {
        if let Some(popup) = self.active_popup.as_mut() {
            match popup {
                ActivePopup::CommandPalette(popup) => popup.select_previous(),
                ActivePopup::ExitConfirmation(_popup) => {}
            };
        } else {
            let Some(index) = self.workspaces_state.selected() else {
                if self.page_number != 0 {
                    self.redirect = Some(Route::Workspaces(routes::workspaces::Route::List(
                        parameters::workspaces::list::Parameters {
                            search_query: self.search_query(),
                            page_number: self.page_number - 1,
                            page_size: self.page_size,
                        },
                    )));
                }

                return;
            };

            if index == 0 {
                if self.page_number == 0 {
                    return;
                }

                self.redirect = Some(Route::Workspaces(routes::workspaces::Route::List(
                    parameters::workspaces::list::Parameters {
                        search_query: self.search_query(),
                        page_number: self.page_number - 1,
                        page_size: self.page_size,
                    },
                )));

                return;
            }

            self.workspaces_state.select_previous();
        }
    }

    fn enter_char(&mut self, c: char) {
        self.search.enter_char(c);

        let route = Route::Workspaces(routes::workspaces::Route::List(
            parameters::workspaces::list::Parameters {
                search_query: self.search_query(),
                page_number: 0,
                page_size: self.page_size,
            },
        ));

        self.redirect = Some(route);
    }

    fn search_query(&self) -> String {
        self.search.value().to_string()
    }

    fn delete_char(&mut self) {
        self.search.delete_char();

        let route = Route::Workspaces(routes::workspaces::Route::List(
            parameters::workspaces::list::Parameters {
                search_query: self.search_query(),
                page_number: 0,
                page_size: self.page_size,
            },
        ));

        self.redirect = Some(route);
    }

    fn delete_all_chars(&mut self) {
        self.search.delete_all_chars();

        let route = Route::Workspaces(routes::workspaces::Route::List(
            parameters::workspaces::list::Parameters {
                search_query: self.search_query(),
                page_number: 0,
                page_size: self.page_size,
            },
        ));

        self.redirect = Some(route);
    }

    fn move_cursor_left(&mut self) {
        self.search.move_cursor_left();
    }

    fn move_cursor_right(&mut self) {
        self.search.move_cursor_right();
    }

    fn workspace(&self) -> Option<&Presenter> {
        self.workspaces_state
            .selected()
            .and_then(|i| self.workspaces.get(i))
    }
}
