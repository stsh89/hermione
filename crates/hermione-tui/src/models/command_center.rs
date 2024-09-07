use crate::{clients::OrganizerClient, data::Workspace, Result};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState, Paragraph},
    Frame,
};
use std::{io::Write, process::Stdio};

use super::{TableauModel, TableauModelParameters};

pub struct Model<'a> {
    state: State,
    selected_command_index: Option<usize>,
    workspace_index: usize,
    organizer: &'a mut OrganizerClient,
}

pub struct ModelParameters<'a> {
    pub workspace_index: usize,
    pub organizer: &'a mut OrganizerClient,
}

enum State {
    Running,
    Exited,
}

pub enum Message {
    CreateCommand(NewCommand),
    DeleteCommand,
    Exit,
    SelectNextCommand,
    SelectPreviousCommand,
}

struct View<'a> {
    command_name: &'a str,
    programs: &'a [String],
    selected_command_index: Option<usize>,
    workspace_name: &'a str,
}

pub struct NewCommand {
    pub name: String,
    pub program: String,
}

impl<'a> Model<'a> {
    pub fn delete_command(&mut self) -> Result<()> {
        let Some(index) = self.selected_command_index else {
            return Ok(());
        };

        self.organizer.delete_command(self.workspace_index, index)?;
        self.selected_command_index = None;

        Ok(())
    }

    pub fn new(params: ModelParameters<'a>) -> Self {
        let ModelParameters {
            workspace_index,
            organizer,
        } = params;

        Self {
            state: State::Running,
            selected_command_index: None,
            workspace_index,
            organizer,
        }
    }

    fn programs(&self) -> Result<Vec<String>> {
        self.workspace().map(|workspace| {
            workspace
                .commands
                .iter()
                .map(|command| command.program.clone())
                .collect()
        })
    }

    fn workspace(&self) -> Result<Workspace> {
        self.organizer.get_workspace(self.workspace_index)
    }

    pub fn update(&mut self, message: Message) -> Result<()> {
        match message {
            Message::SelectPreviousCommand => self.select_prev()?,
            Message::SelectNextCommand => self.select_next()?,
            Message::DeleteCommand => self.delete_command()?,
            Message::Exit => self.exit(),
            Message::CreateCommand(parameters) => self.create_command(parameters)?,
        }

        Ok(())
    }

    fn create_command(&mut self, parameters: NewCommand) -> Result<()> {
        let NewCommand { name, program } = parameters;

        self.organizer
            .create_command(self.workspace_index, name, program)?;
        self.selected_command_index = Some(self.programs()?.len() - 1);

        Ok(())
    }

    fn select_next(&mut self) -> Result<()> {
        if self.workspace()?.commands.is_empty() {
            return Ok(());
        }

        let Some(index) = self.selected_command_index else {
            self.selected_command_index = Some(0);

            return Ok(());
        };

        if index == (self.workspace()?.commands.len() - 1) {
            self.selected_command_index = Some(0);
        } else {
            self.selected_command_index = Some(index + 1);
        }

        Ok(())
    }

    fn select_prev(&mut self) -> Result<()> {
        if self.workspace()?.commands.is_empty() {
            return Ok(());
        }

        let Some(index) = self.selected_command_index else {
            self.selected_command_index = Some(self.workspace()?.commands.len() - 1);

            return Ok(());
        };

        if index == 0 {
            self.selected_command_index = Some(self.workspace()?.commands.len() - 1);
        } else {
            self.selected_command_index = Some(index - 1);
        }

        Ok(())
    }

    pub fn is_exited(&self) -> bool {
        matches!(self.state, State::Exited)
    }

    fn exit(&mut self) {
        self.state = State::Exited;
    }

    pub fn tableau(&mut self) -> Result<Option<TableauModel>> {
        let Some(nidex) = self.selected_command_index else {
            return Ok(None);
        };

        let command = self.organizer.get_command(self.workspace_index, nidex)?;

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

        let model = TableauModel::new(TableauModelParameters {
            command,
            stdout,
            stderr,
        });

        Ok(Some(model))
    }

    pub fn view(&self, frame: &mut Frame) {
        let workspace = self.organizer.get_workspace(self.workspace_index).unwrap();
        let programs: Vec<String> = workspace
            .commands
            .iter()
            .map(|command| command.program.clone())
            .collect();

        let command_name = if let Some(index) = self.selected_command_index {
            let command = &workspace.commands[index];
            &command.name
        } else {
            ""
        };

        let view = View {
            programs: &programs,
            selected_command_index: self.selected_command_index,
            workspace_name: &workspace.name,
            command_name,
        };

        view.render(frame);
    }
}

impl<'a> View<'a> {
    fn render(&self, frame: &mut Frame) {
        let layout = Layout::new(
            Direction::Vertical,
            vec![Constraint::Percentage(100), Constraint::Min(3)],
        )
        .flex(Flex::Start);
        let [top, bottom] = layout.areas(frame.area());

        let list = List::new(self.programs.to_vec())
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

        let paragraph = Paragraph::new(self.command_name).block(
            Block::new()
                .title("Command name")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(paragraph, bottom)
    }
}
