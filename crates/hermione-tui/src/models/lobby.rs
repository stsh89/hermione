use crate::{clients::organizer::Client, entities::Workspace, elements::Selector, Result};
use anyhow::Ok;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState},
    Frame,
};

pub struct Model<'a> {
    selector: Selector<Workspace>,
    state: State,
    organizer: &'a mut Client,
}

pub struct ModelParameters<'a> {
    pub organizer: &'a mut Client,
}

enum State {
    Running,
    Exited,
}

pub enum Message {
    DeleteWorkspace,
    Exit,
    SelectNextWorkspace,
    SelectPreviousWorkspace,
    Save,
}

struct View<'a> {
    selector: &'a Selector<Workspace>,
}

impl Message {
    fn is_idempotent(&self) -> bool {
        match &self {
            Self::Exit | Self::Save | Self::SelectNextWorkspace | Self::SelectPreviousWorkspace => {
                true
            }
            Self::DeleteWorkspace => false,
        }
    }
}

impl<'a> Model<'a> {
    fn delete_workspace(&mut self) -> Result<()> {
        self.organizer.delete_workspace(self.selector.item().id)
    }

    fn exit(&mut self) {
        self.state = State::Exited;
    }

    pub fn is_exited(&self) -> bool {
        matches!(self.state, State::Exited)
    }

    pub fn new(parameters: ModelParameters<'a>) -> Result<Self> {
        let ModelParameters { organizer } = parameters;
        let workspaces = organizer.list_workspaces();

        Ok(Self {
            selector: Selector::new(workspaces)?,
            state: State::Running,
            organizer,
        })
    }

    fn save(&self) -> Result<()> {
        self.organizer.save()
    }

    fn select_next_workspace(&mut self) {
        self.selector.next();
    }

    fn select_previous_workspace(&mut self) {
        self.selector.previous()
    }

    pub fn update(mut self, message: Message) -> Result<Self> {
        let is_idempotent = message.is_idempotent();

        match message {
            Message::DeleteWorkspace => self.delete_workspace()?,
            Message::Exit => self.exit(),
            Message::SelectNextWorkspace => self.select_next_workspace(),
            Message::SelectPreviousWorkspace => self.select_previous_workspace(),
            Message::Save => self.save()?,
        }

        if is_idempotent {
            Ok(self)
        } else {
            let selector = Selector::new(self.organizer.list_workspaces())?;
            Ok(Self { selector, ..self })
        }
    }

    pub fn view(&self, frame: &mut Frame) {
        let view = View {
            selector: &self.selector,
        };

        view.render(frame);
    }

    pub fn workspace(&self) -> &Workspace {
        self.selector.item()
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
        self.selector
            .item()
            .commands
            .iter()
            .map(|command| command.program.clone())
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

        state.select(Some(self.selector.item_number()));

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
