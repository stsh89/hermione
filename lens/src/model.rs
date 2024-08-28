use crate::Message;
use projection::Projection;

pub struct Model {
    is_exited: bool,
    projection: Projection,
    selected_workspace_index: Option<usize>,
    workspace_is_entered: bool,
}

impl Model {
    pub fn is_exited(&self) -> bool {
        self.is_exited
    }

    fn exit(&mut self) {
        self.is_exited = true;
    }

    pub fn initialize(projection: Projection) -> Self {
        Self {
            projection,
            is_exited: false,
            selected_workspace_index: None,
            workspace_is_entered: false,
        }
    }

    pub fn select_workspace(&mut self, index: usize) {
        self.selected_workspace_index = Some(index);
    }

    pub fn selected_workspace_index(&self) -> Option<usize> {
        self.selected_workspace_index
    }

    pub fn enter_workspace(&mut self) {
        self.workspace_is_entered = true;
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Exit => self.exit(),
            Message::SelectWorkspace(index) => self.select_workspace(index),
        }
    }

    pub fn workspace_names(&self) -> Vec<&str> {
        self.projection
            .workspaces()
            .iter()
            .map(|w| w.name())
            .collect()
    }

    pub fn workspace_commands(&self, workspace_index: usize) -> Vec<&str> {
        if let Some(workspace) = self.projection.workspaces().get(workspace_index) {
            workspace
                .instructions()
                .iter()
                .map(|i| i.directive())
                .collect()
        } else {
            vec![]
        }
    }
}
