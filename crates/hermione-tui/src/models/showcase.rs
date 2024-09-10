use crate::{clients::OrganizerClient, data::Workspace, Result};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState},
    Frame,
};

use super::{elements::Selector, CommandCenterModel, CommandCenterModelParameters};

pub struct Model<'a> {
    selector: Selector<Workspace>,
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

struct View<'a> {
    selector: &'a Selector<Workspace>,
}

impl<'a> Model<'a> {
    pub fn command_center(&mut self) -> Result<Option<CommandCenterModel>> {
        let Some(workspace) = self.selector.item() else {
            return Ok(None);
        };

        let model = CommandCenterModel::new(CommandCenterModelParameters {
            workspace: workspace.clone(),
            organizer: self.organizer,
        });

        Ok(Some(model))
    }

    fn save(&self) -> Result<()> {
        self.organizer.save()
    }

    pub fn new(parameters: ModelParameters<'a>) -> Self {
        let ModelParameters { organizer } = parameters;
        let workspaces = organizer.workspaces();

        Self {
            selector: Selector::new(workspaces),
            state: State::Running,
            organizer,
        }
    }

    pub fn view(&self, frame: &mut Frame) {
        let view = View {
            selector: &self.selector,
        };

        view.render(frame);
    }

    fn delete_workspace(&mut self) -> Result<()> {
        if let Some(workspace) = self.selector.item() {
            self.organizer.delete_workspace(workspace.id)?;
            self.reset_selector();
        }

        Ok(())
    }

    fn create_workspace(&mut self, name: String) -> Result<()> {
        self.organizer.create_workspace(name.to_string())?;
        self.reset_selector();

        Ok(())
    }

    pub fn reset_selector(&mut self) {
        self.selector = Selector::new(self.organizer.workspaces());
    }

    fn select_next(&mut self) {
        self.selector.next();
    }

    fn select_prev(&mut self) {
        self.selector.prev()
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

impl<'a> View<'a> {
    fn workspace_names(&self) -> Vec<String> {
        self.selector
            .items()
            .iter()
            .map(|workspace| workspace.name.clone())
            .collect()
    }

    fn programs(&self) -> Vec<String> {
        let Some(workspace) = self.selector.item() else {
            return vec![];
        };

        workspace
            .commands
            .iter()
            .map(|command| command.program.to_string())
            .collect()
    }

    fn render(self, frame: &mut Frame) {
        let layout = Layout::new(
            Direction::Horizontal,
            vec![Constraint::Percentage(25), Constraint::Percentage(75)],
        )
        .flex(Flex::Start);
        let [left, right] = layout.areas(frame.area());

        let list = List::new(self.workspace_names())
            .highlight_style(Style::new().reversed())
            .block(
                Block::new()
                    .title("Workspaces")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::all()),
            );
        let mut state = ListState::default();

        state.select(self.selector.item_number());

        frame.render_stateful_widget(list, left, &mut state);

        let list = List::new(self.programs()).block(
            Block::new()
                .title("Commands")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(list, right)
    }
}
