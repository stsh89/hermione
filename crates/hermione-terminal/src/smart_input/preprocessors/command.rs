#[derive(Default)]
pub struct Preprocessor {
    search_query: String,
    index: Option<usize>,
}

impl Preprocessor {
    pub fn append_search_query(&mut self, c: char) {
        self.index = None;
        self.search_query.push(c);
    }

    pub fn command<'a>(&self, commands: &'a [String]) -> Option<&'a str> {
        let index = self.index?;

        self.filter_commands(commands).into_iter().nth(index)
    }

    fn filter_commands<'a>(&self, commands: &'a [String]) -> Vec<&'a str> {
        let search_query = self.search_query.to_lowercase();

        commands
            .iter()
            .filter(|c| c.to_lowercase().starts_with(&search_query))
            .map(|c| c.as_str())
            .collect()
    }

    pub fn next_command<'a>(&mut self, commands: &'a [String]) -> Option<&'a str> {
        let mut first = None;

        for (index, command) in self.filter_commands(commands).into_iter().enumerate() {
            if index == 0 {
                first = Some(command);
            }

            let Some(i) = self.index else {
                self.index = Some(index);

                return Some(command);
            };

            if index == i + 1 {
                self.index = Some(index);

                return Some(command);
            }
        }

        if first.is_some() {
            self.index = Some(0);
        }

        first
    }

    pub fn update_search_query(&mut self, value: &str) {
        self.index = None;
        self.search_query = value.into();
    }
}
