pub struct Program(String);

impl Program {
    pub fn new(program: String) -> Self {
        Self(program)
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
