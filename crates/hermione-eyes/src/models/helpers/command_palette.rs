use crate::{models::highlight_style, types::Result};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, Clear, List, ListItem, ListState, Padding},
    Frame,
};

pub struct CommandPalette {
    actions_state: ListState,
    actions: Vec<Action>,
    is_active: bool,
}

pub struct CommandPaletteParameters {
    pub actions: Vec<Action>,
}

pub enum Action {
    DeleteCommand,
    DeleteWorkspace,
    EditCommand,
    EditWorkspace,
    ListWorkspaces,
    NewCommand,
    NewWorkspace,
}

impl CommandPalette {
    pub fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn new(parameters: CommandPaletteParameters) -> Result<Self> {
        let CommandPaletteParameters { actions } = parameters;

        if actions.is_empty() {
            return Err(anyhow::anyhow!(
                "Command palette should have at least one action"
            ));
        }

        Ok(Self {
            actions_state: ListState::default().with_selected(Some(0)),
            actions,
            is_active: false,
        })
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered()
            .title("Command palette")
            .padding(Padding::proportional(1));

        let items: Vec<ListItem> = self.actions.iter().map(ListItem::from).collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(highlight_style());

        let area = popup_area(area, 60, 40);
        frame.render_widget(Clear, area);
        frame.render_stateful_widget(list, area, &mut self.actions_state);
    }

    pub fn select_next(&mut self) {
        self.actions_state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.actions_state.select_previous();
    }

    pub fn action(&self) -> Option<&Action> {
        self.actions_state
            .selected()
            .and_then(|index| self.actions.get(index))
    }
}

impl<'a> From<&Action> for ListItem<'a> {
    fn from(action: &Action) -> Self {
        let content = match action {
            Action::DeleteCommand => "Delete command",
            Action::DeleteWorkspace => "Delete workspace",
            Action::EditCommand => "Edit command",
            Action::EditWorkspace => "Edit workspace",
            Action::ListWorkspaces => "List workspaces",
            Action::NewCommand => "New command",
            Action::NewWorkspace => "New workspace",
        };

        ListItem::new(content)
    }
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
