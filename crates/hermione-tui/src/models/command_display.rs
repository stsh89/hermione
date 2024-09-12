use crate::{
    clients::command_executor::{Client, Output},
    entities::Command,
    Result,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Model {
    command: Command,
    stdout: String,
    stderr: String,
    state: State,
}

pub struct ModelParameters {
    pub command: Command,
}

pub enum Message {
    Exit,
    RepeatCommand,
}

enum State {
    Exited,
    Running,
}

struct View<'a> {
    program: &'a str,
    stderr: &'a str,
    stdout: &'a str,
}

impl Model {
    fn execute_command(&mut self) -> Result<()> {
        let Output { stdout, stderr } = Client::new(&self.command).execute()?;

        self.stderr = stderr;
        self.stdout = stdout;

        Ok(())
    }

    fn exit(&mut self) {
        self.state = State::Exited;
    }

    pub fn is_exited(&self) -> bool {
        matches!(self.state, State::Exited)
    }

    pub fn new(parameters: ModelParameters) -> Result<Self> {
        let ModelParameters { command } = parameters;

        let mut model = Self {
            command,
            state: State::Running,
            stderr: String::new(),
            stdout: String::new(),
        };

        model.execute_command()?;

        Ok(model)
    }

    fn repeat_command(&mut self) -> Result<()> {
        self.execute_command()
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
