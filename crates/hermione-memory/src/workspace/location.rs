#[derive(Debug, Default)]
pub struct Location(String);

impl Location {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
