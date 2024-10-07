use hermione_deeds::workspaces::commands::Dto;
use ratatui::widgets::ListItem;

pub struct Presenter {
    pub workspace_id: String,
    pub id: String,
    pub name: String,
    pub program: String,
}

impl<'a> From<&Presenter> for ListItem<'a> {
    fn from(command: &Presenter) -> Self {
        ListItem::new(command.program.clone())
    }
}

impl From<Presenter> for Dto {
    fn from(value: Presenter) -> Self {
        let Presenter {
            workspace_id,
            id,
            name,
            program,
        } = value;

        Dto {
            id,
            last_execute_time: None,
            name,
            program,
            workspace_id,
        }
    }
}

impl From<Dto> for Presenter {
    fn from(value: Dto) -> Self {
        let Dto {
            id,
            last_execute_time: _,
            name,
            program,
            workspace_id,
        } = value;

        Presenter {
            workspace_id,
            id,
            name,
            program,
        }
    }
}
