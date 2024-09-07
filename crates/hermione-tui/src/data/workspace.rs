use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Workspace {
    pub name: String,
    pub commands: Vec<Command>,
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub program: String,
}
