pub struct Id(usize);

impl Id {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn raw(&self) -> usize {
        self.0
    }
}
