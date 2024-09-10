#[derive(Default)]
pub struct Name(String);

impl Name {
    pub fn new(name: String) -> Self {
        Name(name)
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
