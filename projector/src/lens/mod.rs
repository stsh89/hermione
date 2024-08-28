mod context;
mod message;

use context::{Context, WorkspaceContext, WorkspaceFormContext, WorkspacesContext};
use message::Message;
use projection::{Projection, Workspace};
use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    Frame,
};
use std::time::Duration;

pub struct Lens {
    projection: Projection,
    state: State,
    context: Context,
}

enum State {
    Open,
    Closed,
}

impl Lens {
    pub fn is_closed(&self) -> bool {
        matches!(self.state, State::Closed)
    }

    fn close(&mut self) {
        self.state = State::Closed;
    }

    pub fn open(projection: Projection) -> Self {
        let workspaces = projection
            .workspaces()
            .iter()
            .map(|w| w.name().to_string())
            .collect();

        Self {
            projection,
            state: State::Open,
            context: Context::Workspaces(WorkspacesContext::new(workspaces)),
        }
    }

    fn workspace_names(&self) -> Vec<String> {
        self.projection
            .workspaces()
            .iter()
            .map(|w| w.name().to_string())
            .collect()
    }

    fn workspace_commands(&self, workspace_index: usize) -> Vec<String> {
        let Some(workspace) = self.projection.workspaces().get(workspace_index) else {
            return vec![];
        };

        workspace
            .instructions()
            .iter()
            .map(|i| i.directive().to_string())
            .collect()
    }

    pub fn view(&self, frame: &mut Frame) {
        self.context.view(frame);
    }

    pub fn handle_event(&self) -> std::io::Result<Option<Message>> {
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let message = self.context.handle_key(key.code);

                    return Ok(message);
                }
            }
        }

        Ok(None)
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::CloseLens => self.close(),

            Message::SelectNextWorkspace => self.select_next_workspace(),

            Message::SelectPreviousWorkspace => self.select_previous_workspace(),

            Message::EnterWorkspace => self.enter_workspace(),

            Message::ExitWorkspace => self.exit_workspace(),

            Message::SelectNextCommand => self.select_next_command(),

            Message::SelectPreviousCommand => self.select_previous_command(),

            Message::DeleteWorkspace => self.delete_workspace(),

            Message::ExitWorkspaceForm => self.exit_workspace_form(),

            Message::CreateWorkspace => self.create_workspace(),

            Message::WorkspaceFormAddChar(char) => self.workspace_form_add_char(char),

            Message::WorkspaceFormNameDeleteChar => self.workspace_form_delete_char(),

            Message::WorkspaceFormMoveCusorLeft => self.workspace_form_move_cursor_left(),

            Message::WorkspaceFormMoveCusorRight => self.workspace_form_move_cursor_right(),

            Message::EnterWorkspaceForm => self.enter_workspace_form(),
        }
    }

    fn enter_workspace_form(&mut self) {
        self.context = Context::WorkspaceForm(WorkspaceFormContext {
            value: "".to_string(),
            character_index: 0,
        });
    }

    fn workspace_form_move_cursor_left(&mut self) {
        let Context::WorkspaceForm(ref mut context) = self.context else {
            return;
        };

        context.move_cursor_left();
    }

    fn workspace_form_move_cursor_right(&mut self) {
        let Context::WorkspaceForm(ref mut context) = self.context else {
            return;
        };

        context.move_cursor_right();
    }

    fn create_workspace(&mut self) {
        let Context::WorkspaceForm(ref mut context) = self.context else {
            return;
        };

        let workspace = Workspace::new(context.value.clone());

        self.projection.add_workspace(workspace);
        self.context = Context::Workspaces(WorkspacesContext::new(self.workspace_names()));
    }

    fn workspace_form_add_char(&mut self, char: char) {
        let Context::WorkspaceForm(ref mut context) = self.context else {
            return;
        };

        context.enter_char(char);
    }

    fn workspace_form_delete_char(&mut self) {
        let Context::WorkspaceForm(ref mut context) = self.context else {
            return;
        };

        context.delete_char();
    }

    fn delete_workspace(&mut self) {
        let Context::Workspaces(context) = &self.context else {
            return;
        };

        let Some(index) = context.selected_workspace_index else {
            return;
        };

        self.projection.remove_workspace(index);
        self.context = Context::Workspaces(WorkspacesContext::new(self.workspace_names()));
    }

    fn enter_workspace(&mut self) {
        let Context::Workspaces(context) = &self.context else {
            return;
        };

        let Some(workspace_index) = context.selected_workspace_index else {
            return;
        };

        self.context = Context::Workspace(WorkspaceContext {
            workspace_index,
            commands: self.workspace_commands(workspace_index),
            selected_command_index: None,
            selected_command_name: "".to_string(),
            workspace_name: self.workspace_name(workspace_index),
        });
    }

    fn workspace_name(&self, workspace_index: usize) -> String {
        let Some(workspace) = self.projection.workspaces().get(workspace_index) else {
            return "".to_string();
        };

        workspace.name().to_string()
    }

    fn exit_workspace(&mut self) {
        self.context = Context::Workspaces(WorkspacesContext::new(self.workspace_names()));
    }

    fn exit_workspace_form(&mut self) {
        self.context = Context::Workspaces(WorkspacesContext::new(self.workspace_names()));
    }

    fn select_next_workspace(&mut self) {
        let Context::Workspaces(context) = &self.context else {
            return;
        };

        if context.workspaces.is_empty() {
            return;
        }

        let mut new_index = 0;

        if let Some(index) = context.selected_workspace_index {
            if index < (context.workspaces.len() - 1) {
                new_index = index + 1;
            }
        }

        self.context = Context::Workspaces(WorkspacesContext {
            selected_workspace_index: Some(new_index),
            workspaces: self.workspace_names(),
            commands: self.workspace_commands(new_index),
        });
    }

    fn select_next_command(&mut self) {
        let Context::Workspace(context) = &self.context else {
            return;
        };

        if context.commands.is_empty() {
            return;
        }

        let mut new_index = 0;

        if let Some(index) = context.selected_command_index {
            if index < (context.commands.len() - 1) {
                new_index = index + 1;
            }
        }

        self.context = Context::Workspace(WorkspaceContext {
            workspace_index: context.workspace_index,
            selected_command_index: Some(new_index),
            commands: context.commands.clone(),
            selected_command_name: self.command_name(context.workspace_index, new_index),
            workspace_name: context.workspace_name.clone(),
        });
    }

    fn select_previous_command(&mut self) {
        let Context::Workspace(context) = &self.context else {
            return;
        };

        if context.commands.is_empty() {
            return;
        }

        let mut new_index = context.commands.len() - 1;

        if let Some(index) = context.selected_command_index {
            if index > 0 {
                new_index = index - 1;
            }
        }

        self.context = Context::Workspace(WorkspaceContext {
            workspace_index: context.workspace_index,
            selected_command_index: Some(new_index),
            commands: context.commands.clone(),
            selected_command_name: self.command_name(context.workspace_index, new_index),
            workspace_name: context.workspace_name.clone(),
        });
    }

    fn command_name(&self, workspace_index: usize, command_index: usize) -> String {
        let Some(workspace) = self.projection.workspaces().get(workspace_index) else {
            return "".to_string();
        };

        let Some(command) = workspace.instructions().get(command_index) else {
            return "".to_string();
        };

        command.name().to_string()
    }

    fn select_previous_workspace(&mut self) {
        let Context::Workspaces(context) = &self.context else {
            return;
        };

        if context.workspaces.is_empty() {
            return;
        }

        let mut new_index = context.workspaces.len() - 1;

        if let Some(index) = context.selected_workspace_index {
            if index > 0 {
                new_index = index - 1;
            }
        }

        self.context = Context::Workspaces(WorkspacesContext {
            selected_workspace_index: Some(new_index),
            workspaces: self.workspace_names(),
            commands: self.workspace_commands(new_index),
        });
    }
}
