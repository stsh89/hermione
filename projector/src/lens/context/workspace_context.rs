use projection::{Projection, Workspace};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState, Paragraph},
    Frame,
};

pub struct WorkspaceContext {
    pub workspace_index: usize,
    pub selected_command_index: Option<usize>,
    pub commands: Vec<String>,
    pub selected_command_name: String,
    pub workspace_name: String,
}

impl WorkspaceContext {
    pub fn delete_command(&mut self, projection: &mut Projection) {
        let Some(index) = self.selected_command_index else {
            return;
        };

        let Some(workspace) = projection.get_workspace_mut(self.workspace_index) else {
            return;
        };

        workspace.remove_instruction(index);
        self.commands = commands(workspace);
        self.selected_command_name = "".to_string();
        self.selected_command_index = None;
    }

    pub fn render(&self, frame: &mut Frame) {
        let layout = Layout::new(
            Direction::Vertical,
            vec![Constraint::Percentage(100), Constraint::Min(3)],
        )
        .flex(Flex::Start);
        let [top, bottom] = layout.areas(frame.area());

        let list = List::new(self.commands.iter().map(|c| c.as_str()))
            .highlight_style(Style::new().reversed())
            .block(
                Block::new()
                    .title(format!("{} commands", self.workspace_name))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::all()),
            );
        let mut state = ListState::default();

        state.select(self.selected_command_index);

        frame.render_stateful_widget(list, top, &mut state);

        let paragraph = Paragraph::new(self.selected_command_name.as_str()).block(
            Block::new()
                .title("Command name")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(paragraph, bottom)
    }

    pub fn select_next_command(&mut self, projection: &Projection) {
        if self.commands.is_empty() {
            return;
        }

        let mut new_index = 0;

        if let Some(index) = self.selected_command_index {
            if index < (self.commands.len() - 1) {
                new_index = index + 1;
            }
        }

        self.selected_command_index = Some(new_index);
        self.selected_command_name = command_name(projection, self.workspace_index, new_index);
    }

    pub fn select_previous_command(&mut self, projection: &Projection) {
        if self.commands.is_empty() {
            return;
        }

        let mut new_index = self.commands.len() - 1;

        if let Some(index) = self.selected_command_index {
            if index > 0 {
                new_index = index - 1;
            }
        }

        self.selected_command_index = Some(new_index);
        self.selected_command_name = command_name(&projection, self.workspace_index, new_index);
    }
}

fn command_name(projection: &Projection, workspace_index: usize, command_index: usize) -> String {
    projection
        .get_instruction(workspace_index, command_index)
        .map(|i| i.name().to_string())
        .unwrap_or_default()
}

fn commands(workspace: &Workspace) -> Vec<String> {
    workspace
        .instructions()
        .iter()
        .map(|i| i.directive().to_string())
        .collect()
}
