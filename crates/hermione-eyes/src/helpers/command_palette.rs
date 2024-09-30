use super::List;
use crate::types::Result;
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Clear, ListItem, ListState},
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

#[derive(PartialEq)]
pub enum Action {
    CopyToClipboard,
    DeleteCommand,
    DeleteWorkspace,
    EditCommand,
    EditWorkspace,
    ListWorkspaces,
    NewCommand,
    NewWorkspace,
    SetPowershellNoExit,
    StartWindowsTerminal,
    UnsetPowerShellNoExit,
}

impl CommandPalette {
    pub fn action(&self) -> Option<&Action> {
        self.actions_state
            .selected()
            .and_then(|index| self.actions.get(index))
    }

    pub fn hide(&mut self) {
        self.is_active = false;
    }

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
        let area = popup_area(area, 60, 40);
        let list = List {
            title: "Command palette",
            items: &self.actions,
        };

        frame.render_widget(Clear, area);
        frame.render_stateful_widget(list, area, &mut self.actions_state);
    }

    pub fn select_next(&mut self) {
        if let Some(index) = self.actions_state.selected() {
            if index == self.actions.len() - 1 {
                self.actions_state.select_first();
            } else {
                self.actions_state.select_next();
            }
        }
    }

    pub fn select_previous(&mut self) {
        if let Some(index) = self.actions_state.selected() {
            if index == 0 {
                self.actions_state.select_last();
            } else {
                self.actions_state.select_previous();
            }
        }
    }
}

impl<'a> From<&Action> for ListItem<'a> {
    fn from(action: &Action) -> Self {
        let content = match action {
            Action::CopyToClipboard => "Copy to clipboard",
            Action::DeleteCommand => "Delete command",
            Action::DeleteWorkspace => "Delete workspace",
            Action::EditCommand => "Edit command",
            Action::EditWorkspace => "Edit workspace",
            Action::ListWorkspaces => "List workspaces",
            Action::NewCommand => "New command",
            Action::NewWorkspace => "New workspace",
            Action::SetPowershellNoExit => "Set PowerShell -NoExit",
            Action::UnsetPowerShellNoExit => "Unset PowerShell -NoExit",
            Action::StartWindowsTerminal => "Start Windows Terminal",
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
