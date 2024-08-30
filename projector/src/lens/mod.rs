mod context;
mod input;
mod message;

use context::{
    ActiveInput, CommandExecutionContext, CommandFormContext, Context, WorkspaceContext,
    WorkspaceFormContext,
};
use input::Input;
use message::Message;
use projection::{Instruction, InstructionAttributes, Projection, Workspace};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
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
        Self {
            context: Context::workspaces(&projection),
            projection,
            state: State::Open,
        }
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

    fn is_editor_mode(&self) -> bool {
        self.context.is_in_editor_mode()
    }

    pub fn handle_event(&self) -> std::io::Result<Option<Message>> {
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let message = match key.code {
                        KeyCode::Char(to_insert) if self.is_editor_mode() => {
                            Some(Message::EnterChar(to_insert))
                        }
                        KeyCode::Char('q') => Some(Message::Close),
                        KeyCode::Esc => Some(Message::ExitContext),
                        KeyCode::Up => Some(Message::SelectPrevious),
                        KeyCode::Down => Some(Message::SelectNext),
                        KeyCode::Enter => Some(Message::Enter),
                        KeyCode::Char('d') => Some(Message::DeleteSelection),
                        KeyCode::Char('n') => Some(Message::New),
                        KeyCode::Backspace => Some(Message::DeleteChar),
                        KeyCode::Left => Some(Message::MoveCusorLeft),
                        KeyCode::Right => Some(Message::MoveCusorRight),
                        KeyCode::Tab => Some(Message::ToggleActiveInput),
                        _ => None,
                    };

                    return Ok(message);
                }
            }
        }

        Ok(None)
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Close => self.close(),
            Message::DeleteChar => self.delete_char(),
            Message::DeleteSelection => self.delete_selection(),
            Message::Enter => self.confirm(),
            Message::EnterChar(char) => self.enter_char(char),
            Message::ExitContext => self.exit_context(),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::New => self.initialize_form_context(),
            Message::SelectNext => self.select_next(),
            Message::SelectPrevious => self.select_previous(),
            Message::ToggleActiveInput => self.toggle_active_input(),
        }
    }

    fn toggle_active_input(&mut self) {
        self.context.toggle_active_input();
    }

    fn initialize_form_context(&mut self) {
        if let Context::Workspaces(_) = &self.context {
            self.context = Context::WorkspaceForm(WorkspaceFormContext {
                name: Input::default(),
            });
        } else if let Context::Workspace(context) = &self.context {
            self.context = Context::CommandForm(CommandFormContext {
                workspace_index: context.workspace_index,
                name: Input::default(),
                directive: Input::default(),
                active_input: ActiveInput::Directive,
            });
        };
    }

    fn move_cursor_left(&mut self) {
        self.context.move_cursor_left();
    }

    fn move_cursor_right(&mut self) {
        self.context.move_cursor_right();
    }

    fn confirm(&mut self) {
        match &self.context {
            Context::WorkspaceForm(context) => {
                let workspace = Workspace::new(context.name.value.clone());
                self.projection.add_workspace(workspace);
                self.context = Context::workspaces(&self.projection);
            }
            Context::CommandForm(context) => {
                let instruction = Instruction::new(InstructionAttributes {
                    name: context.name.value.clone(),
                    directive: context.directive.value.clone(),
                });

                let Some(workspace) = self.projection.get_workspace_mut(context.workspace_index)
                else {
                    return;
                };

                workspace.add_instruction(instruction);

                self.context = Context::Workspace(WorkspaceContext {
                    workspace_index: context.workspace_index,
                    selected_command_index: None,
                    commands: self.workspace_commands(context.workspace_index),
                    selected_command_name: String::new(),
                    workspace_name: self.workspace_name(context.workspace_index),
                });
            }
            Context::Workspaces(context) => {
                if let Some(workspace_index) = context.selected_workspace_index {
                    self.context = Context::Workspace(WorkspaceContext {
                        workspace_index,
                        commands: self.workspace_commands(workspace_index),
                        selected_command_index: None,
                        selected_command_name: "".to_string(),
                        workspace_name: self.workspace_name(workspace_index),
                    });
                };
            }
            Context::Workspace(context) => {
                let (stdout, stderr) = context.execute_command();

                self.context = Context::CommandExecution(CommandExecutionContext {
                    stdout,
                    stderr,
                    workspace_index: context.workspace_index,
                    command_index: context.selected_command_index.unwrap(),
                    command_name: context.selected_command_name.clone(),
                    command_directive: self
                        .projection
                        .get_instruction(
                            context.workspace_index,
                            context.selected_command_index.unwrap(),
                        )
                        .unwrap()
                        .directive()
                        .to_string(),
                });
            }
            Context::CommandExecution(_context) => {}
        }
    }

    fn enter_char(&mut self, char: char) {
        self.context.enter_char(char);
    }

    fn delete_char(&mut self) {
        self.context.delete_char();
    }

    fn delete_selection(&mut self) {
        if let Context::Workspaces(ref mut context) = self.context {
            context.delete_workspace(&mut self.projection);
        } else if let Context::Workspace(ref mut context) = self.context {
            context.delete_command(&mut self.projection);
        }
    }

    fn workspace_name(&self, workspace_index: usize) -> String {
        let Some(workspace) = self.projection.workspaces().get(workspace_index) else {
            return "".to_string();
        };

        workspace.name().to_string()
    }

    fn exit_context(&mut self) {
        match &self.context {
            Context::Workspaces(_) => self.close(),
            Context::Workspace(_) => self.context = Context::workspaces(&self.projection),
            Context::WorkspaceForm(_) => self.context = Context::workspaces(&self.projection),
            Context::CommandForm(context) => {
                self.context = Context::Workspace(WorkspaceContext {
                    workspace_index: context.workspace_index,
                    selected_command_index: None,
                    commands: self.workspace_commands(context.workspace_index),
                    selected_command_name: String::new(),
                    workspace_name: self.workspace_name(context.workspace_index),
                })
            }
            Context::CommandExecution(context) => {
                self.context = Context::Workspace(WorkspaceContext {
                    workspace_index: context.workspace_index,
                    selected_command_index: Some(context.command_index),
                    commands: self.workspace_commands(context.workspace_index),
                    selected_command_name: context.command_name.clone(),
                    workspace_name: self.workspace_name(context.workspace_index),
                })
            }
        };
    }

    fn select_next(&mut self) {
        if let Context::Workspaces(ref mut context) = &mut self.context {
            context.select_next_workspace(&self.projection);
        } else if let Context::Workspace(ref mut context) = &mut self.context {
            context.select_next_command(&self.projection);
        }
    }

    fn select_previous(&mut self) {
        if let Context::Workspaces(ref mut context) = &mut self.context {
            context.select_previous_workspace(&self.projection);
        } else if let Context::Workspace(ref mut context) = &mut self.context {
            context.select_previous_command(&self.projection);
        }
    }
}
