#[derive(Default)]
pub struct Preprocessor {
    search_query: String,
    index: Option<usize>,
}

impl Preprocessor {
    pub fn command<'a>(&self, commands: &'a [String]) -> Option<&'a str> {
        let index = self.index?;

        self.find_commands(commands).get(index).map(|c| c.as_str())
    }

    pub fn find_command<'a>(&mut self, query: &str, commands: &'a [String]) -> Option<&'a str> {
        self.index = None;
        self.search_query = query.into();

        let found_commands = self.find_commands(commands);

        let command = found_commands.first()?;

        self.index = Some(0);
        Some(command)
    }

    fn find_commands<'a>(&self, commands: &'a [String]) -> Vec<&'a String> {
        let search_query = self.search_query.to_lowercase();

        commands
            .iter()
            .filter(|c| c.to_lowercase().starts_with(&search_query))
            .collect()
    }

    pub fn next_command<'a>(&mut self, commands: &'a [String]) -> Option<&'a str> {
        let found_commands = self.find_commands(commands);

        let Some(index) = self.index else {
            return found_commands.first().map(|c| c.as_str());
        };

        let index = index + 1;
        self.index = Some(index);

        if let Some(command) = found_commands.get(index) {
            return Some(command.as_str());
        }

        self.index = Some(0);

        if let Some(command) = found_commands.first() {
            return Some(command.as_str());
        }

        self.index = None;

        None
    }

    pub fn update_search_query(&mut self, value: &str) {
        self.index = None;
        self.search_query = value.into();
    }
}
