use std::ops::Deref;

pub struct Directive(String);

impl Directive {
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl Deref for Directive {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
