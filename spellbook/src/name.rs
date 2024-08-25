pub struct Name(String);

impl Name {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

impl From<&Name> for String {
    fn from(name: &Name) -> Self {
        name.0.to_string()
    }
}
