use ratatui::widgets::ListItem;

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

impl<'a> From<&Workspace> for ListItem<'a> {
    fn from(workspace: &Workspace) -> Self {
        ListItem::new(workspace.name.clone())
    }
}
