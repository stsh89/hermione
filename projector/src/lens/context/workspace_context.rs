use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState, Paragraph},
    Frame,
};
use std::io::Write;
use std::process::Stdio;

use crate::organizer::Workspace;

pub struct WorkspaceContext {
    pub workspace_index: usize,
    pub selected_command_index: Option<usize>,
}

impl WorkspaceContext {
    pub fn execute_command(&self, workspaces: &[Workspace]) -> (String, String) {
        let Some(index) = self.selected_command_index else {
            return (String::new(), String::new());
        };

        let Some(workspace) = workspaces.get(self.workspace_index) else {
            return (String::new(), String::new());
        };

        let Some(command) = workspace.commands.get(index) else {
            return (String::new(), String::new());
        };

        let mut cmd = std::process::Command::new("PowerShell");
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        let mut process = cmd.spawn().expect("launch failure");
        let stdin = process.stdin.as_mut().expect("pipe failure");
        writeln!(stdin, "{}", command.program).unwrap();
        let out = process.wait_with_output().unwrap();
        let stdout = std::str::from_utf8(out.stdout.as_slice())
            .unwrap()
            .to_string();
        let stderr = std::str::from_utf8(out.stderr.as_slice())
            .unwrap()
            .to_string();

        (stdout, stderr)
    }

    fn commands_list(&self, workspace: &Workspace) -> Vec<String> {
        workspace
            .commands
            .iter()
            .map(|w| w.program.clone())
            .collect()
    }

    pub fn render(&self, frame: &mut Frame, workspaces: &[Workspace]) {
        let layout = Layout::new(
            Direction::Vertical,
            vec![Constraint::Percentage(100), Constraint::Min(3)],
        )
        .flex(Flex::Start);
        let [top, bottom] = layout.areas(frame.area());

        let Some(workspace) = workspaces.get(self.workspace_index) else {
            return;
        };

        let commands = self.commands_list(workspace);

        let list = List::new(commands)
            .highlight_style(Style::new().reversed())
            .block(
                Block::new()
                    .title(format!("{} commands", workspace.name))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::all()),
            );
        let mut state = ListState::default();

        state.select(self.selected_command_index);

        frame.render_stateful_widget(list, top, &mut state);

        let name = if let Some(index) = self.selected_command_index {
            workspace
                .commands
                .get(index)
                .map(|c| c.name.clone())
                .unwrap_or_default()
        } else {
            String::new()
        };

        let paragraph = Paragraph::new(name).block(
            Block::new()
                .title("Command name")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(paragraph, bottom)
    }

    pub fn select_next_command(&mut self, workspaces: &[Workspace]) {
        let Some(workspace) = workspaces.get(self.workspace_index) else {
            return;
        };

        if workspace.commands.is_empty() {
            return;
        }

        let mut new_index = 0;

        if let Some(index) = self.selected_command_index {
            if index < (workspace.commands.len() - 1) {
                new_index = index + 1;
            }
        }

        self.selected_command_index = Some(new_index);
    }

    pub fn select_previous_command(&mut self, workspaces: &[Workspace]) {
        let Some(workspace) = workspaces.get(self.workspace_index) else {
            return;
        };

        if workspace.commands.is_empty() {
            return;
        }

        let mut new_index = workspace.commands.len() - 1;

        if let Some(index) = self.selected_command_index {
            if index > 0 {
                new_index = index - 1;
            }
        }

        self.selected_command_index = Some(new_index);
    }
}
