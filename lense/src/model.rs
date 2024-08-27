use crate::Message;
use projection::{Projection, Workspace};

pub struct Model {
    projection: Projection,
    state: State,
}

pub enum State {
    Initialized,
    WorkspacePreview(usize),
    Exited,
}

impl Model {
    fn exit(&mut self) {
        self.state = State::Exited;
    }

    pub fn initialize() -> Self {
        let mut projection = Projection::default();
        projection.add_workspace(Workspace::new("Hermione".to_string()));
        projection.add_workspace(Workspace::new("General".to_string()));
        projection.add_workspace(Workspace::new("Vulkan tutorial".to_string()));

        Self {
            state: State::Initialized,
            projection,
        }
    }

    pub fn is_exited(&self) -> bool {
        matches!(self.state, State::Exited)
    }

    fn preview_next_workspace(&mut self) {
        if self.projection.workspaces().is_empty() {
            return;
        }

        match self.state {
            State::Initialized => self.state = State::WorkspacePreview(0),
            State::WorkspacePreview(workspace_index) => {
                let index_reset = 0;
                let next_index = workspace_index + 1;

                let index = if next_index == self.projection.workspaces().len() {
                    index_reset
                } else {
                    next_index
                };

                self.state = State::WorkspacePreview(index);
            }
            State::Exited => todo!(),
        }
    }

    fn preview_previous_workspace(&mut self) {
        if self.projection.workspaces().is_empty() {
            return;
        }

        match self.state {
            State::Initialized => {
                self.state = State::WorkspacePreview(self.projection.workspaces().len() - 1)
            }
            State::WorkspacePreview(workspace_index) => {
                let index = if workspace_index == 0 {
                    self.projection.workspaces().len() - 1
                } else {
                    workspace_index - 1
                };

                self.state = State::WorkspacePreview(index);
            }
            State::Exited => todo!(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::AddWorkspace => todo!(),
            Message::EnterWorkspace => todo!(),
            Message::Exit => self.exit(),
            Message::ExitWorkspace => todo!(),
            Message::NewWorkspace => todo!(),
            Message::RemoveWorkspace => todo!(),
            Message::PreviewNextWorkspace => self.preview_next_workspace(),
            Message::PreviewPreviousWorkspace => self.preview_previous_workspace(),
        }
    }

    pub fn workspace_names(&self) -> Vec<&str> {
        self.projection
            .workspaces()
            .iter()
            .map(|w| w.name())
            .collect()
    }

    pub fn current_workspace_preview_index(&self) -> Option<usize> {
        match self.state {
            State::Initialized => None,
            State::WorkspacePreview(index) => Some(index),
            State::Exited => None,
        }
    }
}
