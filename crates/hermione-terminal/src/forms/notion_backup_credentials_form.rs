use crate::{
    themes::{Theme, Themed},
    widgets::TextInputWidget,
    NotionBackupCredentialsPresenter,
};
use hermione_tui::Input;
use ratatui::{
    layout::{Constraint, Direction, Position, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const API_KEY: &str = "Api key";
const COMMANDS_DATABASE_ID: &str = "Commands database id";
const WORKSPACES_DATABASE_ID: &str = "Workspaces database id";

pub struct NotionBackupCredentialsForm {
    api_key: Input,
    active_input: ActiveInput,
    commands_database_id: Input,
    workspaces_database_id: Input,
    theme: Theme,
}

pub struct NotionBackupCredentialsFormParameters {
    pub notion_backup_credentials_presenter: Option<NotionBackupCredentialsPresenter>,
    pub theme: Theme,
}

enum ActiveInput {
    ApiKey,
    CommandsDatabaseId,
    WorkspacesDatabaseId,
}

impl NotionBackupCredentialsForm {
    fn active_input_mut(&mut self) -> &mut Input {
        match self.active_input {
            ActiveInput::ApiKey => &mut self.api_key,
            ActiveInput::CommandsDatabaseId => &mut self.commands_database_id,
            ActiveInput::WorkspacesDatabaseId => &mut self.workspaces_database_id,
        }
    }

    fn active_input(&self) -> &Input {
        match self.active_input {
            ActiveInput::ApiKey => &self.api_key,
            ActiveInput::CommandsDatabaseId => &self.commands_database_id,
            ActiveInput::WorkspacesDatabaseId => &self.workspaces_database_id,
        }
    }

    pub fn credentials(&self) -> NotionBackupCredentialsPresenter {
        NotionBackupCredentialsPresenter {
            api_key: self.api_key.value().to_string(),
            commands_database_id: self.commands_database_id.value().to_string(),
            workspaces_database_id: self.workspaces_database_id.value().to_string(),
        }
    }

    pub fn delete_all_chars(&mut self) {
        self.active_input_mut().delete_all_chars();
    }

    pub fn delete_char(&mut self) {
        self.active_input_mut().delete_char();
    }

    pub fn new(parameters: NotionBackupCredentialsFormParameters) -> Self {
        let NotionBackupCredentialsFormParameters {
            notion_backup_credentials_presenter,
            theme,
        } = parameters;

        let Some(NotionBackupCredentialsPresenter {
            api_key,
            workspaces_database_id,
            commands_database_id,
        }) = notion_backup_credentials_presenter
        else {
            return Self {
                active_input: ActiveInput::ApiKey,
                api_key: Input::default(),
                commands_database_id: Input::default(),
                workspaces_database_id: Input::default(),
                theme,
            };
        };

        Self {
            active_input: ActiveInput::ApiKey,
            api_key: Input::new(api_key),
            commands_database_id: Input::new(commands_database_id),
            workspaces_database_id: Input::new(workspaces_database_id),
            theme,
        }
    }

    pub fn enter_char(&mut self, c: char) {
        self.active_input_mut().enter_char(c);
    }

    fn api_key_input(&self) -> TextInputWidget {
        TextInputWidget::new(self.api_key.value()).themed(self.theme)
    }

    fn commands_database_id_input(&self) -> TextInputWidget {
        TextInputWidget::new(self.commands_database_id.value()).themed(self.theme)
    }

    fn workspaces_database_id_input(&self) -> TextInputWidget {
        TextInputWidget::new(self.workspaces_database_id.value()).themed(self.theme)
    }

    fn api_key_text(&self) -> Paragraph {
        Paragraph::new(self.api_key.value())
    }

    fn commands_database_id_text(&self) -> Paragraph {
        Paragraph::new(self.commands_database_id.value())
    }

    fn workspaces_database_id_text(&self) -> Paragraph {
        Paragraph::new(self.workspaces_database_id.value())
    }

    pub fn move_cursor_left(&mut self) {
        self.active_input_mut().move_cursor_left();
    }

    pub fn move_cursor_right(&mut self) {
        self.active_input_mut().move_cursor_right();
    }

    pub fn select_next_input(&mut self) {
        self.active_input = self.active_input.next();
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let [api_key_area, commands_database_id_area, workspaces_database_id_area] =
            ratatui::layout::Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Max(3),
                    Constraint::Max(3),
                    Constraint::Min(3),
                ])
                .areas(area);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(API_KEY)
            .themed(self.theme);

        match self.active_input {
            ActiveInput::ApiKey => {
                frame.render_widget(self.api_key_input().block(block), api_key_area);
            }
            _ => {
                frame.render_widget(self.api_key_text().block(block), api_key_area);
            }
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(COMMANDS_DATABASE_ID)
            .themed(self.theme);

        match self.active_input {
            ActiveInput::CommandsDatabaseId => frame.render_widget(
                self.commands_database_id_input().block(block),
                commands_database_id_area,
            ),
            _ => frame.render_widget(
                self.commands_database_id_text().block(block),
                commands_database_id_area,
            ),
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(WORKSPACES_DATABASE_ID)
            .themed(self.theme);

        match self.active_input {
            ActiveInput::WorkspacesDatabaseId => frame.render_widget(
                self.workspaces_database_id_input().block(block),
                workspaces_database_id_area,
            ),
            _ => frame.render_widget(
                self.workspaces_database_id_text().block(block),
                workspaces_database_id_area,
            ),
        };

        let active_input_area = match self.active_input {
            ActiveInput::ApiKey => api_key_area,
            ActiveInput::CommandsDatabaseId => commands_database_id_area,
            ActiveInput::WorkspacesDatabaseId => workspaces_database_id_area,
        };

        frame.set_cursor_position(Position::new(
            active_input_area.x + self.active_input().character_index() as u16 + 1,
            active_input_area.y + 1,
        ));
    }
}

impl ActiveInput {
    fn next(&self) -> Self {
        match self {
            ActiveInput::ApiKey => ActiveInput::CommandsDatabaseId,
            ActiveInput::CommandsDatabaseId => ActiveInput::WorkspacesDatabaseId,
            ActiveInput::WorkspacesDatabaseId => ActiveInput::ApiKey,
        }
    }
}
