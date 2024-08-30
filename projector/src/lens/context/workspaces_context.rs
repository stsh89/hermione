use handbag::Organizer;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState},
    Frame,
};

pub struct WorkspacesContext {
    pub selected_workspace_index: Option<usize>,
    pub workspaces: Vec<String>,
    pub commands: Vec<String>,
}

impl WorkspacesContext {
    pub fn delete_workspace(&mut self, organizer: &mut Organizer) {
        let Some(index) = self.selected_workspace_index else {
            return;
        };

        organizer.remove_workspace(index);

        self.reset(organizer);
    }

    fn reset(&mut self, organizer: &Organizer) {
        let new = Self::new(organizer);

        self.selected_workspace_index = new.selected_workspace_index;
        self.commands = new.commands;
        self.workspaces = new.workspaces;
    }

    pub fn new(organizer: &Organizer) -> Self {
        let workspaces = organizer
            .workspaces()
            .iter()
            .map(|w| w.name().to_string())
            .collect();

        Self {
            selected_workspace_index: None,
            commands: vec![],
            workspaces,
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let layout = Layout::new(
            Direction::Horizontal,
            vec![Constraint::Percentage(25), Constraint::Percentage(75)],
        )
        .flex(Flex::Start);
        let [left, right] = layout.areas(frame.area());

        let list = List::new(self.workspaces.iter().map(|w| w.as_str()))
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

        let list = List::new(self.commands.iter().map(|c| c.as_str())).block(
            Block::new()
                .title("Commands")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(list, right)
    }

    pub fn select_next_workspace(&mut self, organizer: &Organizer) {
        if self.workspaces.is_empty() {
            return;
        }

        let mut new_index = 0;

        if let Some(index) = self.selected_workspace_index {
            if index < (self.workspaces.len() - 1) {
                new_index = index + 1;
            }
        }

        self.selected_workspace_index = Some(new_index);
        self.commands = commands(new_index, organizer);
    }

    pub fn select_previous_workspace(&mut self, organizer: &Organizer) {
        if self.workspaces.is_empty() {
            return;
        }

        let mut new_index = self.workspaces.len() - 1;

        if let Some(index) = self.selected_workspace_index {
            if index > 0 {
                new_index = index - 1;
            }
        }

        self.selected_workspace_index = Some(new_index);
        self.commands = commands(new_index, organizer);
    }
}

fn commands(workspace_index: usize, organizer: &Organizer) -> Vec<String> {
    if let Some(workspace) = organizer.get_workspace(workspace_index) {
        workspace
            .instructions()
            .iter()
            .map(|i| i.directive().to_string())
            .collect()
    } else {
        vec![]
    }
}