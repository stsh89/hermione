use crate::{layouts::Popup, widgets, Result};
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders},
    Frame,
};

pub struct Component {
    actions_state: widgets::list::State,
    actions: Vec<Action>,
    scroll_state: widgets::scroll::State,
    scroll: usize,
}

pub struct ComponentParameters {
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

impl Component {
    pub fn action(&self) -> Option<&Action> {
        self.actions_state
            .selected()
            .and_then(|index| self.actions.get(index))
    }

    pub fn new(parameters: ComponentParameters) -> Result<Self> {
        let ComponentParameters { actions } = parameters;

        if actions.is_empty() {
            return Err(anyhow::anyhow!(
                "Command palette should have at least one action"
            ));
        }

        let actions_state = widgets::list::State::default().with_selected(Some(0));
        let mut scroll_state = widgets::scroll::State::default();
        scroll_state = scroll_state.content_length(actions.len());

        Ok(Self {
            actions_state,
            actions,
            scroll_state,
            scroll: 0,
        })
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let area = Popup::default().area(area);

        let block = Block::default().borders(Borders::all());
        let list = widgets::list::Widget::new(&self.actions).block(block);
        let scroll = widgets::scroll::Widget {};

        frame.render_widget(widgets::clear::Widget {}, area);
        frame.render_stateful_widget(list, area, &mut self.actions_state);
        frame.render_stateful_widget(scroll, area, &mut self.scroll_state);
    }

    pub fn select_next(&mut self) {
        let Some(index) = self.actions_state.selected() else {
            return;
        };

        if index == self.actions.len() - 1 {
            self.actions_state.select_first();
            self.scroll = 0;
        } else {
            self.actions_state.select_next();
            self.scroll = self.scroll.saturating_add(1);
        }

        self.scroll_state = self.scroll_state.position(self.scroll);
    }

    pub fn select_previous(&mut self) {
        let Some(index) = self.actions_state.selected() else {
            return;
        };

        if index == 0 {
            self.actions_state.select_last();
            self.scroll = self.actions.len() - 1;
        } else {
            self.actions_state.select_previous();
            self.scroll = self.scroll.saturating_sub(1);
        }

        self.scroll_state = self.scroll_state.position(self.scroll);
    }
}

impl<'a> From<&Action> for widgets::list_item::Widget<'a> {
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

        widgets::list_item::Widget::new(content)
    }
}
