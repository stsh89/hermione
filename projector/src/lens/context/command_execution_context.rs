use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::organizer::Workspace;

pub struct CommandExecutionContext {
    pub stdout: String,
    pub stderr: String,
    pub workspace_index: usize,
    pub command_index: usize,
}

impl CommandExecutionContext {
    pub fn render(&self, frame: &mut Frame, workspaces: &[Workspace]) {
        let Some(command) = workspaces
            .get(self.workspace_index)
            .and_then(|w| w.commands.get(self.command_index))
        else {
            return;
        };

        let layout = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Min(3),
                Constraint::Percentage(75),
                Constraint::Percentage(25),
            ],
        );
        let [program, stdout, stderr] = layout.areas(frame.area());

        let paragraph = Paragraph::new(command.program.as_str()).block(
            Block::new()
                .title(command.name.as_str())
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(paragraph, program);

        let paragraph = Paragraph::new(self.stdout.as_str()).block(
            Block::new()
                .title("Stadnard output")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(paragraph, stdout);

        let paragraph = Paragraph::new(self.stderr.as_str()).block(
            Block::new()
                .title("Stadnard error output")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(paragraph, stderr);
    }
}
