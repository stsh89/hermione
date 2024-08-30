mod context;
mod input;
mod message;

use crate::{
    organizer::{Command, OrganizerCLient, Workspace},
    AppResult,
};
use context::{
    ActiveInput, CommandExecutionContext, CommandFormContext, Context, WorkspaceContext,
    WorkspaceFormContext,
};
use input::Input;
use message::Message;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    Frame,
};
use std::time::Duration;

pub struct Lens {
    client: OrganizerCLient,
    state: State,
    context: Context,
    workspaces: Vec<Workspace>,
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

    pub fn open(client: OrganizerCLient) -> AppResult<Self> {
        let workspaces = client.workspaces()?;

        Ok(Self {
            context: Context::workspaces(),
            client,
            state: State::Open,
            workspaces,
        })
    }

    pub fn view(&self, frame: &mut Frame) {
        self.context.view(frame, &self.workspaces);
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
            Message::ExitContext => self.exit_context_or_close(),
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
                let workspace = Workspace {
                    name: context.name.value.clone(),
                    commands: vec![],
                };
                self.workspaces.push(workspace);
                self.context = Context::workspaces();
            }
            Context::CommandForm(context) => {
                let command = Command {
                    name: context.name.value.clone(),
                    program: context.directive.value.clone(),
                };

                let Some(ref mut workspace) = self.workspaces.get_mut(context.workspace_index)
                else {
                    return;
                };

                workspace.commands.push(command);

                self.context = Context::Workspace(WorkspaceContext {
                    workspace_index: context.workspace_index,
                    selected_command_index: None,
                });
            }
            Context::Workspaces(context) => {
                if let Some(workspace_index) = context.selected_workspace_index {
                    self.context = Context::Workspace(WorkspaceContext {
                        workspace_index,
                        selected_command_index: None,
                    });
                };
            }
            Context::Workspace(context) => {
                let (stdout, stderr) = context.execute_command(&self.workspaces);

                self.context = Context::CommandExecution(CommandExecutionContext {
                    stdout,
                    stderr,
                    workspace_index: context.workspace_index,
                    command_index: context.selected_command_index.unwrap(),
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
        if let Context::Workspaces(context) = &self.context {
            self.delete_workspace(context.selected_workspace_index.unwrap());
        } else if let Context::Workspace(context) = &self.context {
            self.delete_command(
                context.workspace_index,
                context.selected_command_index.unwrap(),
            );
        }
    }

    fn delete_workspace(&mut self, workspace_index: usize) {
        self.workspaces.remove(workspace_index);
        self.context = Context::workspaces();
    }

    fn delete_command(&mut self, workspace_index: usize, command_index: usize) {
        self.workspaces[workspace_index]
            .commands
            .remove(command_index);
        self.context = Context::Workspace(WorkspaceContext {
            workspace_index,
            selected_command_index: None,
        });
    }

    fn exit_context_or_close(&mut self) {
        match &self.context {
            Context::Workspaces(_) => self.close(),
            Context::Workspace(_) => self.context = Context::workspaces(),
            Context::WorkspaceForm(_) => self.context = Context::workspaces(),
            Context::CommandForm(context) => {
                self.context = Context::Workspace(WorkspaceContext {
                    workspace_index: context.workspace_index,
                    selected_command_index: None,
                })
            }
            Context::CommandExecution(context) => {
                self.context = Context::Workspace(WorkspaceContext {
                    workspace_index: context.workspace_index,
                    selected_command_index: Some(context.command_index),
                })
            }
        };
    }

    fn select_next(&mut self) {
        self.context.select_next(&self.workspaces);
    }

    fn select_previous(&mut self) {
        self.context.select_previous(&self.workspaces)
    }
}
