use uuid::Uuid;

#[derive(Default)]
pub struct State {
    pub mode: Mode,
    pub list: List,
    pub context: Context,
}

#[derive(Default, Clone, Copy)]
pub enum Context {
    #[default]
    Workspaces,
    Commands {
        workspace_id: Uuid,
    },
}

#[derive(Default)]
pub struct List {
    pub items: Vec<ListItem>,
    pub cursor: usize,
    pub filter: String,
    pub element: usize,
}


pub struct ListItem {
    pub id: Uuid,
    pub text: String,
}

#[derive(Default, Clone, Copy)]
pub enum Mode {
    #[default]
    Normal,
    Input,
}
