use std::ops::Deref;

pub struct Name(String);

impl Name {
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl Deref for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
