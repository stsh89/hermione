use crate::{keyboard, terminal};
use hermione_drive::{Engine, ServiceFactory};
use hermione_nexus::{
    definitions::{Command, CommandId, Workspace, WorkspaceId},
    operations::{
        ExecuteCommandOperation, ListCommandsOperation, ListCommandsParameters,
        ListWorkspacesOperation, ListWorkspacesParameters,
    },
};
use uuid::Uuid;

#[derive(Default)]
pub struct State {
    pub mode: Mode,
    pub list: List,
    pub context: Context,
}

#[derive(Default, Clone, Copy)]
pub enum Context {
    #[default]
    Workspaces,
    Commands {
        workspace_id: Uuid,
    },
}

#[derive(Default)]
pub struct List {
    pub items: Vec<ListItem>,
    pub cursor: usize,
    pub filter: String,
    pub element: usize,
}

pub struct ListItem {
    pub id: Uuid,
    pub text: String,
}

#[derive(Default, Clone, Copy)]
pub enum Mode {
    #[default]
    Normal,
    Input,
}

pub fn run() -> anyhow::Result<()> {
    terminal::install_panic_hook();

    let Engine {
        service_factory,
        logs_worker_guard: _logs_worker_guard,
    } = hermione_drive::start()?;

    let mut terminal = terminal::init()?;
    let mut state = State::default();

    state.list.items = list_workspaces(&state, &service_factory)?;

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

fn exit(state: &State, event: keyboard::Event) -> bool {
    if matches!(state.mode, Mode::Input) {
        return false;
    }

    if matches!(event, keyboard::Event::Char('q')) {
        return true;
    }

    false
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
        | keyboard::Event::Space
        | keyboard::Event::Down
        | keyboard::Event::Backspace
        | keyboard::Event::Enter => false,
    }
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
            keyboard::Event::Enter => maybe_change_context(state, services)?,
            keyboard::Event::Backspace => restore_previous_context(state, services)?,
            keyboard::Event::Space => {
                run_command(state, services, RunCommandOptions { no_exit: false })?
            }
            keyboard::Event::Esc | keyboard::Event::Char(_) => {}
        },
        Mode::Input => match event {
            keyboard::Event::Char(c) => {
                update_active_input(state, InputUpdate::AddChar(c), services)?
            }

            keyboard::Event::Backspace => {
                update_active_input(state, InputUpdate::DeleteChar, services)?
            }

            keyboard::Event::Esc
            | keyboard::Event::Space
            | keyboard::Event::Down
            | keyboard::Event::Up
            | keyboard::Event::Enter => {}
        },
    };

    Ok(())
}

enum InputUpdate {
    AddChar(char),
    DeleteChar,
}

struct RunCommandOptions {
    no_exit: bool,
}

fn run_command(
    state: &mut State,
    services: &ServiceFactory,
    options: RunCommandOptions,
) -> anyhow::Result<()> {
    let Context::Commands { .. } = state.context else {
        return Ok(());
    };

    if state.list.items.is_empty() {
        return Ok(());
    }

    let command_id = CommandId::new(state.list.items[state.list.cursor].id)?;

    let RunCommandOptions { no_exit } = options;

    let storage = services.storage();

    let mut system = services.system();
    system.set_no_exit(no_exit);

    ExecuteCommandOperation {
        find_command_provider: &storage,
        find_workspace_provider: &storage,
        system_provider: &system,
        track_command_provider: &storage,
        track_workspace_provider: &storage,
    }
    .execute(command_id)?;

    Ok(())
}

fn restore_previous_context(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    match state.context {
        Context::Workspaces => {}
        Context::Commands { .. } => {
            state.context = Context::Workspaces;
            state.list.cursor = 0;
            state.list.filter = String::new();
            state.list.items = list_workspaces(state, services)?;
        }
    };

    Ok(())
}

fn maybe_change_context(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    match state.context {
        Context::Workspaces => {
            if state.list.items.is_empty() {
                return Ok(());
            }

            state.context = Context::Commands {
                workspace_id: state.list.items[state.list.cursor].id,
            };

            state.list.cursor = 0;
            state.list.filter = String::new();
            state.list.items = list_commands(state, services)?;
        }
        Context::Commands { .. } => {}
    };

    Ok(())
}

fn update_active_input(
    state: &mut State,
    update: InputUpdate,
    services: &ServiceFactory,
) -> anyhow::Result<()> {
    match update {
        InputUpdate::AddChar(c) => state.list.filter.push(c),
        InputUpdate::DeleteChar => {
            state.list.filter.pop();
        }
    }

    match state.context {
        Context::Workspaces => state.list.items = list_workspaces(state, services)?,
        Context::Commands { .. } => state.list.items = list_commands(state, services)?,
    };

    Ok(())
}

fn select_next_list_item(state: &mut State) {
    state.list.cursor = (state.list.cursor + 1) % state.list.items.len();
}

fn select_previous_list_item(state: &mut State) {
    state.list.cursor = (state.list.cursor + state.list.items.len() - 1) % state.list.items.len();
}

struct DrawOperation<'a, T> {
    renderer: &'a mut T,
}

pub trait Render {
    fn render(&mut self, state: &State) -> anyhow::Result<()>;
}

impl<'a, T> DrawOperation<'a, T>
where
    T: Render,
{
    fn execute(&mut self, state: &State) -> anyhow::Result<()> {
        self.renderer.render(state)
    }
}

fn list_workspaces(state: &State, services: &ServiceFactory) -> anyhow::Result<Vec<ListItem>> {
    let workspaces = ListWorkspacesOperation {
        provider: &services.storage(),
    }
    .execute(ListWorkspacesParameters {
        name_contains: Some(&state.list.filter),
        page_number: None,
        page_size: None,
    })?;

    Ok(workspaces.into_iter().map(Into::into).collect())
}

fn list_commands(state: &State, services: &ServiceFactory) -> anyhow::Result<Vec<ListItem>> {
    let Context::Commands { workspace_id } = state.context else {
        return Ok(Vec::new());
    };

    let commands = ListCommandsOperation {
        provider: &services.storage(),
    }
    .execute(ListCommandsParameters {
        page_size: None,
        page_number: None,
        program_contains: Some(&state.list.filter),
        workspace_id: Some(WorkspaceId::new(workspace_id)?),
    })?;

    Ok(commands.into_iter().map(Into::into).collect())
}

impl From<Workspace> for ListItem {
    fn from(value: Workspace) -> Self {
        ListItem {
            id: value.id().as_uuid(),
            text: value.name().to_string(),
        }
    }
}

impl From<Command> for ListItem {
    fn from(value: Command) -> Self {
        ListItem {
            id: value.id().as_uuid(),
            text: value.program().to_string(),
        }
    }
}
