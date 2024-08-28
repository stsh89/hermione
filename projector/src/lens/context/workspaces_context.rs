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
    pub fn new(workspaces: Vec<String>) -> Self {
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
}
