use hermione_coordinator::workspaces::Dto;
use ratatui::widgets::ListItem;

pub struct Presenter {
    pub id: String,
    pub location: Option<String>,
    pub name: String,
}

impl<'a> From<&Presenter> for ListItem<'a> {
    fn from(workspace: &Presenter) -> Self {
        ListItem::new(workspace.name.clone())
    }
}

impl From<Presenter> for Dto {
    fn from(value: Presenter) -> Self {
        let Presenter { id, location, name } = value;

        Dto {
            id,
            last_access_time: None,
            location,
            name,
        }
    }
}

impl From<Dto> for Presenter {
    fn from(value: Dto) -> Self {
        let Dto {
            id,
            last_access_time: _,
            location,
            name,
        } = value;

        Presenter { id, location, name }
    }
}
