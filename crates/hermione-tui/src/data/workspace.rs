pub struct Workspace {
    pub name: String,
    pub commands: Vec<Command>,
}

pub struct Command {
    pub name: String,
    pub program: String,
}
