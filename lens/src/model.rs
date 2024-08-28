use crate::{input::Input, Message};
use projection::{Instruction, Projection, Workspace};

pub struct Model {
    is_exited: bool,
    projection: Projection,
    selected_workspace_index: Option<usize>,
    selected_command_index: Option<usize>,
    workspace_is_entered: bool,
    new_workspace_input: Option<Input>,
}

impl Model {
    pub fn is_exited(&self) -> bool {
        self.is_exited
    }

    pub fn new_workspace_input(&self) -> Option<&Input> {
        self.new_workspace_input.as_ref()
    }

    fn create_workspace(&mut self) {
        if let Some(input) = &self.new_workspace_input {
            self.projection
                .add_workspace(Workspace::new(input.value().to_string()));
        }

        self.unselect_workspace();
    }

    fn delete_workspace(&mut self) {
        if let Some(index) = self.selected_workspace_index {
            self.projection.remove_workspace(index);
            self.unselect_workspace();
        }
    }

    fn exit(&mut self) {
        self.is_exited = true;
    }

    pub fn initialize(projection: Projection) -> Self {
        Self {
            projection,
            is_exited: false,
            selected_workspace_index: None,
            selected_command_index: None,
            workspace_is_entered: false,
            new_workspace_input: None,
        }
    }

    fn prepare_workspace(&mut self) {
        self.new_workspace_input = Some(Input::default());
    }

    fn cancel_new_workspace(&mut self) {
        self.new_workspace_input = None;
    }

    pub fn new_workspace_is_prepared(&self) -> bool {
        self.new_workspace_input.is_some()
    }

    pub fn select_workspace(&mut self, index: usize) {
        self.selected_workspace_index = Some(index);
    }

    pub fn select_command(&mut self, index: usize) {
        self.selected_command_index = Some(index);
    }

    pub fn selected_workspace(&self) -> Option<&Workspace> {
        if let Some(index) = self.selected_workspace_index {
            return self.projection.workspaces().get(index);
        }

        None
    }

    pub fn selected_workspace_index(&self) -> Option<usize> {
        self.selected_workspace_index
    }

    pub fn selected_command_index(&self) -> Option<usize> {
        self.selected_command_index
    }

    pub fn selected_workspace_commands(&self) -> Vec<&str> {
        if let Some(workspace) = self.selected_workspace() {
            workspace
                .instructions()
                .iter()
                .map(|i| i.directive())
                .collect()
        } else {
            vec![]
        }
    }

    pub fn selected_command(&self) -> Option<&Instruction> {
        if let Some(workspace) = self.selected_workspace() {
            if let Some(index) = self.selected_command_index {
                return workspace.instructions().get(index);
            }
        }

        None
    }

    fn enter_workspace(&mut self) {
        self.workspace_is_entered = true;
    }

    fn exit_workspace(&mut self) {
        self.workspace_is_entered = false;
        self.selected_command_index = None;
    }

    fn unselect_workspace(&mut self) {
        self.selected_workspace_index = None;
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Exit => self.exit(),
            Message::SelectWorkspace(index) => self.select_workspace(index),
            Message::EnterWorkspace => self.enter_workspace(),
            Message::ExitWorkspace => self.exit_workspace(),
            Message::SelectCommand(index) => self.select_command(index),
            Message::UnselectWorkspace => self.unselect_workspace(),
            Message::DeleteWorkspace => self.delete_workspace(),
            Message::NewWorkspace => self.prepare_workspace(),
            Message::CreateWorkspace(_name) => self.create_workspace(),
            Message::CancelNewWorkspace => self.cancel_new_workspace(),
        }
    }

    pub fn workspace_names(&self) -> Vec<&str> {
        self.projection
            .workspaces()
            .iter()
            .map(|w| w.name())
            .collect()
    }

    pub fn workspace_is_entered(&self) -> bool {
        self.workspace_is_entered
    }

    pub fn workspace_is_selected(&self) -> bool {
        self.selected_workspace_index.is_some()
    }
}
