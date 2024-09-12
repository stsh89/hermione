pub struct Workspace {
    pub id: usize,
    pub name: String,
    pub commands: Vec<Command>,
}

pub struct Command {
    pub id: usize,
    pub name: String,
    pub program: String,
}
