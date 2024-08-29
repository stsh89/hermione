use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct CommandExecutionContext {
    pub stdout: String,
    pub stderr: String,
    pub workspace_index: usize,
    pub command_index: usize,
    pub command_name: String,
    pub command_directive: String,
}

impl CommandExecutionContext {
    pub fn render(&self, frame: &mut Frame) {
        let layout = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Min(3),
                Constraint::Percentage(75),
                Constraint::Percentage(25),
            ],
        );
        let [directive, stdout, stderr] = layout.areas(frame.area());

        let paragraph = Paragraph::new(self.command_directive.as_str()).block(
            Block::new()
                .title(self.command_name.as_str())
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(paragraph, directive);

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
