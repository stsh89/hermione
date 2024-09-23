use ratatui::widgets::ListItem;

pub struct Workspace {
    pub name: String,
}

impl<'a> From<&Workspace> for ListItem<'a> {
    fn from(workspace: &Workspace) -> Self {
        ListItem::new(workspace.name.clone())
    }
}
