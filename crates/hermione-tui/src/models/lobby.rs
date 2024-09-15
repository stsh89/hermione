use crate::{
    clients::organizer::Client, elements::Selector, entities::Workspace, session::Session, Result,
};
use anyhow::Ok;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState},
    Frame,
};

pub struct Model<'a> {
    selector: Selector<Workspace>,
    organizer: &'a mut Client,
    session: &'a mut Session,
    signal: Option<Signal>,
}

pub struct ModelParameters<'a> {
    pub organizer: &'a mut Client,
    pub session: &'a mut Session,
}

pub enum Signal {
    EnterCommandCenter(usize),
    NewWorkspaceRequest,
    Exit,
}

pub enum Message {
    DeleteWorkspace,
    EnterCommandCenter,
    NewWorkspaceRequest,
    Exit,
    SelectNextWorkspace,
    SelectPreviousWorkspace,
}

struct View<'a> {
    selector: &'a Selector<Workspace>,
}

impl Message {
    fn is_idempotent(&self) -> bool {
        match &self {
            Self::Exit
            | Self::SelectNextWorkspace
            | Self::SelectPreviousWorkspace
            | Self::NewWorkspaceRequest
            | Self::EnterCommandCenter => true,
            Self::DeleteWorkspace => false,
        }
    }
}

impl<'a> Model<'a> {
    fn delete_workspace(&mut self) -> Result<()> {
        self.organizer.delete_workspace(self.selector.item().id)
    }

    pub fn is_running(&self) -> bool {
        self.signal.is_none()
    }

    pub fn new(parameters: ModelParameters<'a>) -> Result<Self> {
        let ModelParameters { organizer, session } = parameters;
        let workspaces = organizer.list_workspaces();

        Ok(Self {
            selector: Selector::new(workspaces)?,
            signal: None,
            organizer,
            session,
        })
    }

    fn select_next_workspace(&mut self) -> Result<()> {
        self.selector.next();
        self.session
            .set_workspace_id(Some(self.selector.item().id))?;

        Ok(())
    }

    fn select_previous_workspace(&mut self) -> Result<()> {
        self.selector.previous();
        self.session
            .set_workspace_id(Some(self.selector.item().id))?;

        Ok(())
    }

    pub unsafe fn signal(self) -> Signal {
        self.signal.unwrap()
    }

    pub fn update(mut self, message: Message) -> Result<Self> {
        let is_idempotent = message.is_idempotent();

        match message {
            Message::DeleteWorkspace => self.delete_workspace()?,
            Message::Exit => self.signal = Some(Signal::Exit),
            Message::SelectNextWorkspace => self.select_next_workspace()?,
            Message::SelectPreviousWorkspace => self.select_previous_workspace()?,
            Message::NewWorkspaceRequest => self.signal = Some(Signal::NewWorkspaceRequest),
            Message::EnterCommandCenter => {
                self.signal = Some(Signal::EnterCommandCenter(self.selector.item().id))
            }
        }

        if is_idempotent {
            return Ok(self);
        }

        let workspaces = self.organizer.list_workspaces();

        let model = if workspaces.is_empty() {
            Self {
                signal: Some(Signal::NewWorkspaceRequest),
                ..self
            }
        } else {
            let selector = Selector::new(workspaces)?;
            Self { selector, ..self }
        };

        Ok(model)
    }

    pub fn view(&self, frame: &mut Frame) {
        let view = View {
            selector: &self.selector,
        };

        view.render(frame);
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
