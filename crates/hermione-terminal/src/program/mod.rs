mod integration;
mod output;
mod state;

use integration::RunCommandOptions;
pub use output::Render;
pub use state::*;

use crate::{keyboard, terminal};
use hermione_drive::{Engine, ServiceFactory};
use output::DrawOperation;

pub fn run() -> anyhow::Result<()> {
    terminal::install_panic_hook();

    let Engine {
        service_factory,
        logs_worker_guard: _logs_worker_guard,
    } = hermione_drive::start()?;

    let mut terminal = terminal::init()?;
    let mut state = State::default();

    state.list.items = integration::list_workspaces(&state, &service_factory)?;

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

enum ChangeContextMethod {
    Edit,
    NewItem,
    Select,
}

fn change_mode(mode: &mut Mode, event: keyboard::Event) -> bool {
    match event {
        keyboard::Event::Esc => {
            if matches!(mode, Mode::Normal) {
                return false;
            }

            *mode = Mode::Normal;
            true
        }
        keyboard::Event::Char(c) => {
            if matches!(mode, Mode::Input) {
                return false;
            }

            if c == 'i' {
                *mode = Mode::Input;

                return true;
            }

            false
        }
        keyboard::Event::Up
        | keyboard::Event::Tab
        | keyboard::Event::Space
        | keyboard::Event::Down
        | keyboard::Event::Backspace
        | keyboard::Event::Enter => false,
    }
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
        Context::WorkspaceForm { .. } | Context::CommandForm { .. } => {
            state.form.cursor = (state.form.cursor + 1) % state.form.inputs.len();
        }
        Context::Commands { .. } => {}
    }
}

fn maybe_change_context(
    state: &mut State,
    services: &ServiceFactory,
    method: ChangeContextMethod,
) -> anyhow::Result<()> {
    match state.context {
        Context::Workspaces => match method {
            ChangeContextMethod::Edit => {
                let Some(workspace) = integration::get_workspace(state, services)? else {
                    return Ok(());
                };

                state.context = Context::WorkspaceForm {
                    workspace_id: Some(workspace.id().as_uuid()),
                };

                state.form = Form::default();
                state.form.inputs = vec![
                    workspace.name().to_string(),
                    workspace.location().unwrap_or_default().to_string(),
                ];
            }
            ChangeContextMethod::NewItem => {
                state.context = Context::WorkspaceForm { workspace_id: None };

                state.form = Form::default();
                state.form.inputs = vec![String::new(), String::new()];
            }
            ChangeContextMethod::Select => {
                if state.list.items.is_empty() {
                    return Ok(());
                }

                state.context = Context::Commands {
                    workspace_id: state.list.items[state.list.cursor].id,
                };

                state.list.cursor = 0;
                state.list.filter = String::new();
                state.list.items = integration::list_commands(state, services)?;
            }
        },
        Context::Commands { workspace_id } => match method {
            ChangeContextMethod::Select => {}
            ChangeContextMethod::NewItem => {
                state.context = Context::CommandForm {
                    workspace_id,
                    command_id: None,
                };

                state.form = Form::default();
                state.form.inputs = vec![String::new(), String::new()];
            }
            ChangeContextMethod::Edit => {
                let Some(command) = integration::get_command(state, services)? else {
                    return Ok(());
                };

                state.context = Context::CommandForm {
                    workspace_id,
                    command_id: Some(state.list.items[state.list.cursor].id),
                };

                state.form = Form::default();
                state.form.inputs = vec![command.name().to_string(), command.program().to_string()];
            }
        },
        Context::WorkspaceForm { .. } => {
            integration::save_workspace(state, services)?;

            state.context = Context::Workspaces;
            state.list.cursor = 0;
            state.list.filter = String::new();
            state.list.items = integration::list_workspaces(state, services)?;
        }

        Context::CommandForm { workspace_id, .. } => {
            integration::save_command(state, services)?;

            state.context = Context::Commands { workspace_id };
            state.list.cursor = 0;
            state.list.filter = String::new();
            state.list.items = integration::list_commands(state, services)?;
        }
    };

    Ok(())
}

fn maybe_delete_list_item(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    match state.context {
        Context::Workspaces => {
            integration::delete_workspace(state, services)?;

            state.list.cursor = 0;
            state.list.items = integration::list_workspaces(state, services)?;
        }
        Context::WorkspaceForm { .. } => {}
        Context::Commands { .. } => {
            integration::delete_command(state, services)?;

            state.list.cursor = 0;
            state.list.items = integration::list_commands(state, services)?;
        }
        Context::CommandForm { .. } => {}
    };

    Ok(())
}

fn restore_previous_context(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    match state.context {
        Context::Workspaces => {}
        Context::Commands { .. } => {
            state.context = Context::Workspaces;
            state.list.cursor = 0;
            state.list.filter = String::new();
            state.list.items = integration::list_workspaces(state, services)?;
        }
        Context::WorkspaceForm { .. } => {
            state.context = Context::Workspaces;
            state.list.cursor = 0;
            state.list.filter = String::new();
            state.list.items = integration::list_workspaces(state, services)?;
        }
        Context::CommandForm { workspace_id, .. } => {
            state.context = Context::Commands { workspace_id };
            state.list.cursor = 0;
            state.list.filter = String::new();
            state.list.items = integration::list_commands(state, services)?;
        }
    };

    Ok(())
}

fn select_next_list_item(state: &mut State) {
    state.list.cursor = (state.list.cursor + 1) % state.list.items.len();
}

fn select_previous_list_item(state: &mut State) {
    state.list.cursor = (state.list.cursor + state.list.items.len() - 1) % state.list.items.len();
}

fn update_active_input(
    state: &mut State,
    update: InputUpdate,
    services: &ServiceFactory,
) -> anyhow::Result<()> {
    let active_input = match state.context {
        Context::Workspaces | Context::Commands { .. } => &mut state.list.filter,
        Context::WorkspaceForm { .. } | Context::CommandForm { .. } => {
            &mut state.form.inputs[state.form.cursor]
        }
    };

    match update {
        InputUpdate::AddChar(c) => active_input.push(c),
        InputUpdate::DeleteChar => {
            active_input.pop();
        }
    };

    match state.context {
        Context::Workspaces => {
            state.list.cursor = 0;
            state.list.items = integration::list_workspaces(state, services)?;
        }
        Context::Commands { .. } => {
            state.list.cursor = 0;
            state.list.items = integration::list_commands(state, services)?;
        }
        Context::WorkspaceForm { .. } => {}
        Context::CommandForm { .. } => {}
    };

    Ok(())
}

fn update_state(
    state: &mut State,
    event: keyboard::Event,
    services: &ServiceFactory,
) -> anyhow::Result<()> {
    if change_mode(&mut state.mode, event) {
        match state.mode {
            Mode::Normal => state.list.element = 0,
            Mode::Input => state.list.element = 1,
        }

        return Ok(());
    }

    match state.mode {
        Mode::Normal => match event {
            keyboard::Event::Down => select_next_list_item(state),
            keyboard::Event::Up => select_previous_list_item(state),
            keyboard::Event::Enter => {
                maybe_change_context(state, services, ChangeContextMethod::Select)?
            }
            keyboard::Event::Backspace => restore_previous_context(state, services)?,
            keyboard::Event::Space => {
                integration::run_command(state, services, RunCommandOptions { no_exit: false })?
            }
            keyboard::Event::Char(c) => match c {
                'n' => maybe_change_context(state, services, ChangeContextMethod::NewItem)?,
                'd' => maybe_delete_list_item(state, services)?,
                'j' => select_next_list_item(state),
                'k' => select_previous_list_item(state),
                'e' => maybe_change_context(state, services, ChangeContextMethod::Edit)?,
                _ => {}
            },
            keyboard::Event::Esc | keyboard::Event::Tab => {}
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

            keyboard::Event::Enter => {
                update_active_input(state, InputUpdate::AddChar('\n'), services)?
            }

            keyboard::Event::Esc | keyboard::Event::Down | keyboard::Event::Up => {}
        },
    };

    Ok(())
}
