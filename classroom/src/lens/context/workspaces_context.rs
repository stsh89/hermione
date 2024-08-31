use crate::organizer::Workspace;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState},
    Frame,
};

pub struct WorkspacesContext {
    pub selected_workspace_index: Option<usize>,
}

impl WorkspacesContext {
    fn unselect_workspace(&mut self) {
        self.selected_workspace_index = None;
    }

    pub fn initialize() -> Self {
        Self {
            selected_workspace_index: None,
        }
    }

    fn workspace_names_list(workspaces: &[Workspace]) -> Vec<&str> {
        workspaces.iter().map(|w| w.name.as_str()).collect()
    }

    fn command_programs_list(&self, workspaces: &[Workspace]) -> Vec<String> {
        let Some(index) = self.selected_workspace_index else {
            return Vec::new();
        };

        let Some(workspace) = workspaces.get(index) else {
            return Vec::new();
        };

        workspace
            .commands
            .iter()
            .map(|w| w.program.clone())
            .collect()
    }

    pub fn render(&self, frame: &mut Frame, workspaces: &[Workspace]) {
        let layout = Layout::new(
            Direction::Horizontal,
            vec![Constraint::Percentage(25), Constraint::Percentage(75)],
        )
        .flex(Flex::Start);
        let [left, right] = layout.areas(frame.area());

        let list = List::new(Self::workspace_names_list(workspaces))
            .highlight_style(Style::new().reversed())
            .block(
                Block::new()
                    .title("Workspaces")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::all()),
            );
        let mut state = ListState::default();

        state.select(self.selected_workspace_index);

        frame.render_stateful_widget(list, left, &mut state);

        let list = List::new(self.command_programs_list(workspaces)).block(
            Block::new()
                .title("Commands")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(list, right)
    }

    pub fn select_next_workspace(&mut self, workspaces: &[Workspace]) {
        if workspaces.is_empty() {
            return;
        }

        let mut new_index = 0;

        if let Some(index) = self.selected_workspace_index {
            if index < (workspaces.len() - 1) {
                new_index = index + 1;
            }
        }

        self.selected_workspace_index = Some(new_index);
    }

    pub fn select_previous_workspace(&mut self, workspaces: &[Workspace]) {
        if workspaces.is_empty() {
            return;
        }

        let mut new_index = workspaces.len() - 1;

        if let Some(index) = self.selected_workspace_index {
            if index > 0 {
                new_index = index - 1;
            }
        }

        self.selected_workspace_index = Some(new_index);
    }
}
