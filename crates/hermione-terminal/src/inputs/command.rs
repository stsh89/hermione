use crate::smart_input::InputContract;
use hermione_tui::input::Input;

#[derive(Default)]
pub struct CommandInput {
    input: Input,
    search_query: String,
    index: Option<usize>,
    commands: Vec<String>,
    prefix: char,
}

pub struct NewCommandInputParameters {
    pub commands: Vec<String>,
    pub prefix: char,
}

impl InputContract for CommandInput {
    fn delete_char(&mut self) {
        self.input.delete_char();

        let value = self
            .input
            .value()
            .strip_prefix(self.prefix)
            .unwrap_or_default();

        self.search_query = value.into();
        self.index = None;
    }

    fn enter_char(&mut self, c: char) {
        self.index = None;
        self.reset_input();
        self.search_query.push(c);

        let commands = found_commands(&self.search_query, &self.commands);

        let value = if let Some(command) = commands.first() {
            self.index = Some(0);
            command
        } else {
            &self.search_query
        };

        value.chars().for_each(|c| self.input.enter_char(c));
    }

    fn input(&self) -> &Input {
        &self.input
    }

    fn is_empty(&self) -> bool {
        self.input.value().is_empty()
    }

    fn reset(&mut self) {
        self.input.delete_all_chars();
        self.input.enter_char(self.prefix);
        self.search_query.clear();
        self.index = None;
    }

    fn toggle_input(&mut self) {
        self.reset_input();

        let Some(index) = self.index else {
            let commands = found_commands(&self.search_query, &self.commands);

            let value = if let Some(command) = commands.first() {
                self.index = Some(0);
                command
            } else {
                &self.search_query
            };

            value.chars().for_each(|c| self.input.enter_char(c));

            return;
        };

        let index = index + 1;
        self.index = Some(index);

        let commands = found_commands(&self.search_query, &self.commands);

        if let Some(command) = commands.get(index) {
            command.chars().for_each(|c| self.input.enter_char(c));

            return;
        }

        self.index = Some(0);

        if let Some(command) = commands.first() {
            command.chars().for_each(|c| self.input.enter_char(c));

            return;
        }

        self.index = None;
    }

    fn value(&self) -> Option<&str> {
        let Some(index) = self.index else {
            return None;
        };

        found_commands(&self.search_query, &self.commands)
            .get(index)
            .map(|s| s.as_str())
    }
}

impl CommandInput {
    pub fn new(parameters: NewCommandInputParameters) -> Self {
        let NewCommandInputParameters { commands, prefix } = parameters;

        let mut input = Self {
            commands,
            prefix,
            ..Self::default()
        };

        input.input.enter_char(prefix);

        input
    }

    fn reset_input(&mut self) {
        self.input.delete_all_chars();
        self.input.enter_char(self.prefix);
    }
}

fn found_commands<'a>(search_query: &'a str, commands: &'a [String]) -> Vec<&'a String> {
    let search_query = search_query.to_lowercase();

    commands
        .iter()
        .filter(move |c| c.to_lowercase().starts_with(&search_query))
        .collect()
}
