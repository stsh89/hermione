mod integration;

use hermione_nexus::definitions::BackupCredentials;
use integration::RunCommandOptions;

use crate::{
    keyboard,
    program_lib::{Context, Form, List, Mode, Notice, NoticeKind, Render, State},
    terminal,
};
use hermione_drive::{Engine, ServiceFactory};

pub fn run() -> anyhow::Result<()> {
    if let Err(err) = do_run() {
        tracing::error!(error = ?err);

        return Err(err);
    };

    Ok(())
}

struct DrawOperation<'a, T> {
    pub renderer: &'a mut T,
}

impl<T> DrawOperation<'_, T>
where
    T: Render,
{
    fn execute(&mut self, state: &State) -> anyhow::Result<()> {
        self.renderer.render(state)
    }
}

fn do_run() -> anyhow::Result<()> {
    terminal::install_panic_hook();

    let Engine {
        service_factory,
        logs_worker_guard: _logs_worker_guard,
    } = hermione_drive::start()?;

    let mut terminal = terminal::init()?;
    let mut state = State::default();

    setup_workspaces_context(&mut state, &service_factory)?;

    loop {
        DrawOperation {
            renderer: &mut terminal,
        }
        .execute(&state)?;

        let event = keyboard::read_event()?;

        if exit(&state, event) {
            break;
        }

        update_state(&mut state, event, &service_factory)?;
    }

    terminal::restore()?;

    Ok(())
}

enum InputUpdate {
    AddChar(char),
    DeleteChar,
}

fn setup_workspaces_context(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    *state = State {
        context: Context::Workspaces,
        list: List {
            items: integration::list_workspaces(state, services)?,
            ..Default::default()
        },
        ..Default::default()
    };

    if !state.list.items.is_empty() {
        state.workspace_id = Some(state.list.items[0].id);
    };

    Ok(())
}

fn setup_commands_context(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    *state = State {
        workspace_id: state.workspace_id,
        context: Context::Commands,
        list: List {
            items: integration::list_commands(state, services)?,
            ..Default::default()
        },
        ..State::default()
    };

    if !state.list.items.is_empty() {
        state.command_id = Some(state.list.items[0].id);
    };

    Ok(())
}

fn maybe_submit_form(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    match state.context {
        Context::Workspaces => {}
        Context::WorkspaceForm { .. } => {
            integration::save_workspace(state, services)?;
            setup_workspaces_context(state, services)?;
        }
        Context::Commands { .. } => {}
        Context::CommandForm => {
            integration::save_command(state, services)?;
            setup_commands_context(state, services)?;
        }
        Context::NotionBackupCredentialsForm => {
            match integration::save_notion_backup_credentials(state, services) {
                Ok(_) => {
                    state.notice = Some(Notice {
                        message: "Backup credentials saved".to_string(),
                        kind: NoticeKind::Success,
                    });
                }
                Err(err) => {
                    state.notice = Some(Notice {
                        message: err.to_string(),
                        kind: NoticeKind::Error,
                    });
                }
            }
        }
    };

    Ok(())
}

fn exit(state: &State, event: keyboard::Event) -> bool {
    if matches!(state.mode, Mode::Input) {
        return false;
    }

    if matches!(event, keyboard::Event::Char('q')) {
        return true;
    }

    false
}

fn focus_next_input(state: &mut State) {
    match state.context {
        Context::Workspaces => {}
        Context::WorkspaceForm { .. }
        | Context::CommandForm { .. }
        | Context::NotionBackupCredentialsForm => {
            state.form.cursor = (state.form.cursor + 1) % state.form.inputs.len();
        }
        Context::Commands { .. } => {}
    }
}

fn maybe_follow_selected_item(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    let Context::Workspaces = state.context else {
        return Ok(());
    };

    if state.workspace_id.is_none() {
        return Ok(());
    }

    state.list.filter = String::new();
    setup_commands_context(state, services)?;

    Ok(())
}

fn maybe_backup(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    match state.context {
        Context::Workspaces => match integration::backup_workspace(state, services) {
            Ok(_) => {
                state.notice = Some(Notice {
                    message: "Workspace backed up".to_string(),
                    kind: NoticeKind::Success,
                });
            }
            Err(err) => {
                state.notice = Some(Notice {
                    message: err.to_string(),
                    kind: NoticeKind::Error,
                });
            }
        },
        Context::WorkspaceForm { .. } => {}
        Context::Commands { .. } => match integration::backup_command(state, services) {
            Ok(_) => {
                state.notice = Some(Notice {
                    message: "Command backed up".to_string(),
                    kind: NoticeKind::Success,
                });
            }
            Err(err) => {
                state.notice = Some(Notice {
                    message: err.to_string(),
                    kind: NoticeKind::Error,
                });
            }
        },
        Context::CommandForm { .. } => {}
        Context::NotionBackupCredentialsForm => {
            match integration::backup_workspaces(services) {
                Ok(_) => {
                    state.notice = Some(Notice {
                        message: "Workspaces backed up".to_string(),
                        kind: NoticeKind::Success,
                    });
                }
                Err(err) => {
                    state.notice = Some(Notice {
                        message: err.to_string(),
                        kind: NoticeKind::Error,
                    });

                    return Ok(());
                }
            };

            match integration::backup_commands(services) {
                Ok(_) => {
                    state.notice = Some(Notice {
                        message: "Commands backed up".to_string(),
                        kind: NoticeKind::Success,
                    });
                }
                Err(err) => {
                    state.notice = Some(Notice {
                        message: err.to_string(),
                        kind: NoticeKind::Error,
                    });
                }
            };
        }
    };

    Ok(())
}

fn maybe_copy_item(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    integration::copy_command_to_clipboard(state, services)
}

fn maybe_restore(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    if let Context::NotionBackupCredentialsForm = state.context {
        match integration::restore_workspaces(services) {
            Ok(_) => {
                state.notice = Some(Notice {
                    message: "Workspaces backed up".to_string(),
                    kind: NoticeKind::Success,
                });
            }
            Err(err) => {
                state.notice = Some(Notice {
                    message: err.to_string(),
                    kind: NoticeKind::Error,
                });
            }
        };

        match integration::restore_commands(services) {
            Ok(_) => {
                state.notice = Some(Notice {
                    message: "Commands backed up".to_string(),
                    kind: NoticeKind::Success,
                });
            }
            Err(err) => {
                state.notice = Some(Notice {
                    message: err.to_string(),
                    kind: NoticeKind::Error,
                });
            }
        };
    };

    Ok(())
}

fn maybe_edit_item(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    match state.context {
        Context::Workspaces => {
            let Some(workspace) = integration::get_workspace(state, services)? else {
                return Ok(());
            };

            *state = State {
                workspace_id: Some(workspace.id().as_uuid()),
                context: Context::WorkspaceForm,
                form: Form {
                    inputs: vec![
                        workspace.name().to_string(),
                        workspace.location().unwrap_or_default().to_string(),
                    ],
                    ..Default::default()
                },
                ..State::default()
            };
        }
        Context::WorkspaceForm { .. } => {}
        Context::Commands => {
            let Some(command) = integration::get_command(state, services)? else {
                return Ok(());
            };

            *state = State {
                workspace_id: Some(command.workspace_id().as_uuid()),
                command_id: Some(command.id().as_uuid()),
                context: Context::CommandForm,
                form: Form {
                    inputs: vec![command.name().to_string(), command.program().to_string()],
                    ..Default::default()
                },
                ..State::default()
            };
        }
        Context::CommandForm { .. } => {}
        Context::NotionBackupCredentialsForm => {}
    }

    Ok(())
}

fn maybe_new_item(state: &mut State) -> anyhow::Result<()> {
    match state.context {
        Context::Workspaces => {
            *state = State {
                context: Context::WorkspaceForm,
                form: Form {
                    inputs: vec![String::new(), String::new()],
                    ..Default::default()
                },
                ..Default::default()
            };
        }
        Context::WorkspaceForm { .. } => {}
        Context::Commands => {
            *state = State {
                context: Context::CommandForm,
                workspace_id: state.workspace_id,
                form: Form {
                    inputs: vec![String::new(), String::new()],
                    ..Default::default()
                },
                ..Default::default()
            };
        }
        Context::CommandForm { .. } => {}
        Context::NotionBackupCredentialsForm => {}
    };

    Ok(())
}

fn maybe_delete_list_item(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    match state.context {
        Context::Workspaces => {
            integration::delete_workspace(state, services)?;
            setup_workspaces_context(state, services)?;
        }
        Context::WorkspaceForm { .. } => {}
        Context::Commands { .. } => {
            integration::delete_command(state, services)?;

            state.list.cursor = 0;
            state.list.items = integration::list_commands(state, services)?;
        }
        Context::CommandForm { .. } => {}
        Context::NotionBackupCredentialsForm => {}
    };

    Ok(())
}

fn open_terminal(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    integration::open_terminal(state, services)
}

fn restore_parent_context(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    match state.context {
        Context::Workspaces => {}
        Context::CommandForm => {
            state.list.filter = String::new();
            setup_commands_context(state, services)?;
        }
        Context::Commands | Context::NotionBackupCredentialsForm | Context::WorkspaceForm => {
            state.list.filter = String::new();
            setup_workspaces_context(state, services)?
        }
    };

    Ok(())
}

fn select_next_list_item(state: &mut State) {
    match state.context {
        Context::Workspaces => {
            if state.list.items.is_empty() {
                state.workspace_id = None;
            } else {
                state.list.cursor = (state.list.cursor + 1) % state.list.items.len();
                state.workspace_id = Some(state.list.items[state.list.cursor].id);
            }
        }
        Context::Commands => {
            if state.list.items.is_empty() {
                state.command_id = None;
            } else {
                state.list.cursor = (state.list.cursor + 1) % state.list.items.len();
                state.command_id = Some(state.list.items[state.list.cursor].id);
            }
        }
        Context::CommandForm | Context::WorkspaceForm | Context::NotionBackupCredentialsForm => {}
    }
}

fn select_previous_list_item(state: &mut State) {
    match state.context {
        Context::Workspaces => {
            if state.list.items.is_empty() {
                state.workspace_id = None;
            } else {
                state.list.cursor =
                    (state.list.cursor + state.list.items.len() - 1) % state.list.items.len();
                state.workspace_id = Some(state.list.items[state.list.cursor].id);
            }
        }
        Context::Commands => {
            if state.list.items.is_empty() {
                state.command_id = None;
            } else {
                state.list.cursor =
                    (state.list.cursor + state.list.items.len() - 1) % state.list.items.len();
                state.command_id = Some(state.list.items[state.list.cursor].id);
            }
        }
        Context::CommandForm | Context::WorkspaceForm | Context::NotionBackupCredentialsForm => {}
    }
}

fn update_active_input(
    state: &mut State,
    update: InputUpdate,
    services: &ServiceFactory,
) -> anyhow::Result<()> {
    let active_input = match state.context {
        Context::Workspaces | Context::Commands { .. } => &mut state.list.filter,
        Context::WorkspaceForm { .. }
        | Context::CommandForm { .. }
        | Context::NotionBackupCredentialsForm => &mut state.form.inputs[state.form.cursor],
    };

    match update {
        InputUpdate::AddChar(c) => active_input.push(c),
        InputUpdate::DeleteChar => {
            active_input.pop();
        }
    };

    let active_input = active_input.clone();

    match state.context {
        Context::Workspaces => {
            setup_workspaces_context(state, services)?;
            state.list.filter = active_input;
            state.mode = Mode::Input;
        }
        Context::Commands { .. } => {
            setup_commands_context(state, services)?;
            state.list.filter = active_input;
            state.mode = Mode::Input;
        }
        Context::WorkspaceForm { .. } => {}
        Context::CommandForm { .. } => {}
        Context::NotionBackupCredentialsForm => {}
    };

    Ok(())
}

fn update_state(
    state: &mut State,
    event: keyboard::Event,
    services: &ServiceFactory,
) -> anyhow::Result<()> {
    match state.mode {
        Mode::Normal => match event {
            keyboard::Event::Down => select_next_list_item(state),
            keyboard::Event::Up => select_previous_list_item(state),
            keyboard::Event::Enter => maybe_submit_form(state, services)?,
            keyboard::Event::Right => maybe_follow_selected_item(state, services)?,
            keyboard::Event::Left => restore_parent_context(state, services)?,
            keyboard::Event::Space => {
                integration::run_command(state, services, RunCommandOptions { no_exit: false })?
            }
            keyboard::Event::BackSlash => {
                integration::run_command(state, services, RunCommandOptions { no_exit: true })?
            }
            keyboard::Event::Slash => match state.context {
                Context::Workspaces | Context::Commands => state.mode = Mode::Input,
                Context::WorkspaceForm
                | Context::CommandForm
                | Context::NotionBackupCredentialsForm => {}
            },
            keyboard::Event::NumberOne => {
                state.notice = None;
                state.context = Context::NotionBackupCredentialsForm;
                state.form = Form::default();

                if let Some(BackupCredentials::Notion(credentials)) =
                    integration::get_notion_backup_credentials(services)?
                {
                    state.form.inputs = vec![
                        credentials.api_key().to_string(),
                        credentials.commands_database_id().to_string(),
                        credentials.workspaces_database_id().to_string(),
                    ];
                } else {
                    state.form.inputs = vec![String::new(), String::new(), String::new()];
                };
            }
            keyboard::Event::Char(c) => match c {
                'b' => maybe_backup(state, services)?,
                'c' => maybe_copy_item(state, services)?,
                'd' => maybe_delete_list_item(state, services)?,
                'e' => maybe_edit_item(state, services)?,
                'j' => select_next_list_item(state),
                'k' => select_previous_list_item(state),
                'n' => maybe_new_item(state)?,
                'r' => maybe_restore(state, services)?,
                'i' => state.mode = Mode::Input,
                't' => open_terminal(state, services)?,
                _ => {}
            },
            keyboard::Event::Esc | keyboard::Event::Tab | keyboard::Event::Backspace => {}
        },
        Mode::Input => match event {
            keyboard::Event::Char(c) => {
                update_active_input(state, InputUpdate::AddChar(c), services)?
            }

            keyboard::Event::Backspace => {
                update_active_input(state, InputUpdate::DeleteChar, services)?
            }

            keyboard::Event::Tab => {
                focus_next_input(state);
            }

            keyboard::Event::Space => {
                update_active_input(state, InputUpdate::AddChar(' '), services)?
            }

            keyboard::Event::Slash => {
                update_active_input(state, InputUpdate::AddChar('/'), services)?
            }

            keyboard::Event::BackSlash => {
                update_active_input(state, InputUpdate::AddChar('\\'), services)?
            }

            keyboard::Event::NumberOne => {
                update_active_input(state, InputUpdate::AddChar('1'), services)?
            }

            keyboard::Event::Enter => match state.context {
                Context::CommandForm => {
                    if state.form.cursor == 1 {
                        update_active_input(state, InputUpdate::AddChar('\n'), services)?
                    }
                }
                Context::Workspaces => {}
                Context::WorkspaceForm => {}
                Context::Commands => {}
                Context::NotionBackupCredentialsForm => {}
            },

            keyboard::Event::Esc => state.mode = Mode::Normal,

            keyboard::Event::Down
            | keyboard::Event::Up
            | keyboard::Event::Left
            | keyboard::Event::Right => {}
        },
    };

    Ok(())
}
