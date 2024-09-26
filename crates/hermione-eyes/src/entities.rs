use ratatui::widgets::ListItem;

pub struct Workspace {
    pub commands: Vec<Command>,
    pub id: Option<String>,
    pub location: String,
    pub name: String,
}

pub struct Command {
    pub workspace_id: String,
    pub id: Option<String>,
    pub name: String,
    pub program: String,
}

impl Workspace {
    pub fn id(&self) -> &str {
        self.id.as_deref().unwrap_or_default()
    }
}

impl Command {
    pub fn id(&self) -> &str {
        self.id.as_deref().unwrap_or_default()
    }
}

impl<'a> From<&Workspace> for ListItem<'a> {
    fn from(workspace: &Workspace) -> Self {
        ListItem::new(workspace.name.clone())
    }
}

impl<'a> From<&Command> for ListItem<'a> {
    fn from(command: &Command) -> Self {
        ListItem::new(command.program.clone())
    }
}
