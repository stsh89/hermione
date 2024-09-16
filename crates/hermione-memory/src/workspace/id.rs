#[derive(Debug, Default)]
pub struct Id(usize);

impl Id {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn raw(&self) -> usize {
        self.0
    }
}

impl PartialEq<usize> for Id {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}
