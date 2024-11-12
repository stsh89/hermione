mod preprocessors;

use crate::{themes::Theme, tui::Input};
use ratatui::{
    layout::Rect,
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const COMMAND_PREFIX: char = '>';

pub struct SmartInput {
    commands: Vec<String>,
    input: Input,
    preprocessor: Option<Preprocessor>,
    theme: Theme,
}

enum Preprocessor {
    Command(preprocessors::command::Preprocessor),
}

impl Preprocessor {
    fn command() -> Self {
        Self::Command(preprocessors::command::Preprocessor::default())
    }
}

pub struct NewSmartInputParameters {
    pub commands: Vec<String>,
    pub theme: Theme,
}

impl SmartInput {
    pub fn autocomplete(&mut self) {
        let Some(preprocessor) = self.preprocessor.as_mut() else {
            return;
        };

        match preprocessor {
            Preprocessor::Command(preprocessor) => {
                let Some(command) = preprocessor.next_command(self.commands.as_slice()) else {
                    return;
                };

                self.update_command_input(command.into());
            }
        }
    }

    pub fn command(&self) -> Option<&str> {
        match self.preprocessor.as_ref()? {
            Preprocessor::Command(preprocessor) => preprocessor.command(&self.commands),
        }
    }

    pub fn delete_char(&mut self) {
        self.input.delete_char();

        if self.input.value().is_empty() {
            self.preprocessor = None;
        }

        let Some(preprocessor) = self.preprocessor.as_mut() else {
            return;
        };

        match preprocessor {
            Preprocessor::Command(preprocessor) => {
                let value = command_search(&self.input).unwrap_or_default();
                preprocessor.update_search_query(value);
            }
        };
    }

    pub fn enter_char(&mut self, c: char) {
        self.input.enter_char(c);

        if self.input.value().len() == 1 && c == COMMAND_PREFIX {
            self.preprocessor = Some(Preprocessor::command());

            return;
        }

        let Some(preprocessor) = self.preprocessor.as_mut() else {
            return;
        };

        match preprocessor {
            Preprocessor::Command(preprocessor) => {
                preprocessor.append_search_query(c);

                let value = if let Some(command) = preprocessor.next_command(&self.commands) {
                    command.into()
                } else {
                    preprocessor.search_query().into()
                };

                self.update_command_input(value);
            }
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.preprocessor.is_some() {
            return;
        }

        self.input.move_cursor_left();
    }

    pub fn move_cursor_right(&mut self) {
        if self.preprocessor.is_some() {
            return;
        }

        self.input.move_cursor_right();
    }

    pub fn new(parameters: NewSmartInputParameters) -> Self {
        let NewSmartInputParameters { commands, theme } = parameters;

        Self {
            commands,
            input: Input::default(),
            preprocessor: None,
            theme,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Console")
            .bg(self.theme.background_color)
            .fg(self.theme.input_color);

        let paragraph = Paragraph::new(self.input.value()).block(block);

        self.input.render(frame, area, paragraph);
    }

    pub fn reset_input(&mut self) {
        self.input.delete_all_chars();

        let Some(preprocessor) = self.preprocessor.as_mut() else {
            return;
        };

        match preprocessor {
            Preprocessor::Command(preprocessor) => {
                preprocessor.update_search_query("");
                self.input.enter_char(COMMAND_PREFIX);
            }
        }
    }

    pub fn search(&self) -> Option<&str> {
        if self.preprocessor.is_some() {
            return None;
        }

        Some(self.input.value())
    }

    fn update_command_input(&mut self, command: String) {
        self.input.delete_all_chars();
        self.input.enter_char(COMMAND_PREFIX);
        command.chars().for_each(|c| self.input.enter_char(c));
    }
}

pub fn command_search(input: &Input) -> Option<&str> {
    input.value().strip_prefix(COMMAND_PREFIX)
}
