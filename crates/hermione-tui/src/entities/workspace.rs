pub struct Workspace {
    pub number: usize,
    pub name: String,
    pub commands: Vec<Command>,
}

pub struct Command {
    pub number: usize,
    pub name: String,
    pub program: String,
}
