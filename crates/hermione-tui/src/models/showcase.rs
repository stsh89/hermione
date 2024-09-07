use crate::{clients::OrganizerClient, Result};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState},
    Frame,
};

use super::{CommandCenterModel, CommandCenterModelParameters};

pub struct Model<'a> {
    selected_workspace_index: Option<usize>,
    state: State,
    organizer: &'a mut OrganizerClient,
}

pub struct ModelParameters<'a> {
    pub organizer: &'a mut OrganizerClient,
}

enum State {
    Running,
    Exited,
}

pub enum Message {
    CreateWorkspace(String),
    DeleteWorkspace,
    Exit,
    SelectNextWorkspace,
    SelectPreviousWorkspace,
    Save,
}

struct View {
    selected_workspace_index: Option<usize>,
    workspaces: Vec<String>,
    programs: Vec<Vec<String>>,
}

impl<'a> Model<'a> {
    pub fn command_center(&mut self) -> Option<CommandCenterModel> {
        let workspace_index = self.selected_workspace_index?;

        Some(CommandCenterModel::new(CommandCenterModelParameters {
            workspace_index,
            organizer: self.organizer,
        }))
    }

    fn save(&self) -> Result<()> {
        self.organizer.save()
    }

    pub fn new(parameters: ModelParameters<'a>) -> Self {
        let ModelParameters { organizer } = parameters;

        Self {
            selected_workspace_index: None,
            state: State::Running,
            organizer,
        }
    }

    pub fn workspaces(&self) -> Vec<String> {
        self.organizer
            .workspaces()
            .iter()
            .map(|workspace| workspace.name.clone())
            .collect()
    }

    pub fn programs(&self) -> Vec<Vec<String>> {
        self.organizer
            .workspaces()
            .iter()
            .map(|workspace| {
                workspace
                    .commands
                    .iter()
                    .map(|command| command.program.to_string())
                    .collect()
            })
            .collect()
    }

    pub fn view(&self, frame: &mut Frame) {
        let view = View {
            selected_workspace_index: self.selected_workspace_index,
            workspaces: self.workspaces(),
            programs: self.programs(),
        };

        view.render(frame);
    }

    fn delete_workspace(&mut self) -> Result<()> {
        if let Some(index) = self.selected_workspace_index {
            self.organizer.delete_workspace(index)?;
            self.selected_workspace_index = None;
        }

        Ok(())
    }

    fn create_workspace(&mut self, name: String) -> Result<()> {
        self.organizer.create_workspace(name.to_string())?;

        self.selected_workspace_index = Some(self.organizer.workspaces().len() - 1);

        Ok(())
    }

    fn select_next(&mut self) {
        if self.organizer.workspaces().is_empty() {
            return;
        }

        let Some(index) = self.selected_workspace_index else {
            self.selected_workspace_index = Some(0);

            return;
        };

        if index == (self.organizer.workspaces().len() - 1) {
            self.selected_workspace_index = Some(0);
        } else {
            self.selected_workspace_index = Some(index + 1);
        }
    }

    fn select_prev(&mut self) {
        if self.organizer.workspaces().is_empty() {
            return;
        }

        let Some(index) = self.selected_workspace_index else {
            self.selected_workspace_index = Some(self.organizer.workspaces().len() - 1);

            return;
        };

        if index == 0 {
            self.selected_workspace_index = Some(self.organizer.workspaces().len() - 1);
        } else {
            self.selected_workspace_index = Some(index - 1);
        }
    }

    pub fn is_exited(&self) -> bool {
        matches!(self.state, State::Exited)
    }

    fn exit(&mut self) {
        self.state = State::Exited;
    }

    pub fn update(&mut self, message: Message) -> Result<()> {
        match message {
            Message::CreateWorkspace(name) => self.create_workspace(name)?,
            Message::DeleteWorkspace => self.delete_workspace()?,
            Message::Exit => self.exit(),
            Message::SelectNextWorkspace => self.select_next(),
            Message::SelectPreviousWorkspace => self.select_prev(),
            Message::Save => self.save()?,
        }

        Ok(())
    }
}

impl View {
    fn workspace_programs(&self) -> &[String] {
        if let Some(index) = self.selected_workspace_index {
            &self.programs[index]
        } else {
            &[]
        }
    }

    fn render(self, frame: &mut Frame) {
        let layout = Layout::new(
            Direction::Horizontal,
            vec![Constraint::Percentage(25), Constraint::Percentage(75)],
        )
        .flex(Flex::Start);
        let [left, right] = layout.areas(frame.area());

        let list = List::new(self.workspaces.to_vec())
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

        let list = List::new(self.workspace_programs().to_vec()).block(
            Block::new()
                .title("Commands")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(list, right)
    }
}
