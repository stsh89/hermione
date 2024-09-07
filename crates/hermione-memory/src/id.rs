pub struct Id(usize);

impl Id {
    pub fn new(id: usize) -> Self {
        Id(id)
    }

    pub(crate) fn raw(&self) -> usize {
        self.0
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
