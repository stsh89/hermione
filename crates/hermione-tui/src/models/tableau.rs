use crate::{
    clients::{CommandExecutor, CommandExecutorOutput},
    data::Command,
    Result,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Model<'a> {
    command: &'a Command,
    stdout: String,
    stderr: String,
    state: State,
}

pub struct ModelParameters<'a> {
    pub command: &'a Command,
    pub stdout: String,
    pub stderr: String,
}

pub enum Message {
    RepeatCommand,
    Exit,
}

enum State {
    Running,
    Exited,
}

struct View<'a> {
    program: &'a str,
    stdout: &'a str,
    stderr: &'a str,
}

impl<'a> Model<'a> {
    fn exit(&mut self) {
        self.state = State::Exited;
    }

    pub fn is_exited(&self) -> bool {
        matches!(self.state, State::Exited)
    }

    pub fn new(parameters: ModelParameters<'a>) -> Self {
        let ModelParameters {
            command,
            stdout,
            stderr,
        } = parameters;

        Self {
            stdout,
            stderr,
            command,
            state: State::Running,
        }
    }

    fn repeat_command(&mut self) -> Result<()> {
        let CommandExecutorOutput { stdout, stderr } =
            CommandExecutor::new(self.command).execute()?;

        self.stderr = stderr;
        self.stdout = stdout;

        Ok(())
    }

    pub fn update(&mut self, message: Message) -> Result<()> {
        match message {
            Message::RepeatCommand => self.repeat_command()?,
            Message::Exit => self.exit(),
        }

        Ok(())
    }

    pub fn view(&self, frame: &mut Frame) {
        let view = View {
            program: &self.command.program,
            stdout: &self.stdout,
            stderr: &self.stderr,
        };

        view.render(frame);
    }
}

impl<'a> View<'a> {
    fn render(self, frame: &mut Frame) {
        let layout = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Min(3),
                Constraint::Percentage(75),
                Constraint::Percentage(25),
            ],
        );
        let [program, stdout, stderr] = layout.areas(frame.area());

        let paragraph = Paragraph::new(self.program).block(
            Block::new()
                .title(self.program)
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(paragraph, program);

        let paragraph = Paragraph::new(self.stdout).block(
            Block::new()
                .title("Stadnard output")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(paragraph, stdout);

        let paragraph = Paragraph::new(self.stderr).block(
            Block::new()
                .title("Stadnard error output")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(paragraph, stderr);
    }
}
