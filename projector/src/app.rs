use std::time::Duration;

use projection::{Instruction, InstructionAttributes, Projection, Workspace};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState},
    Frame,
};

use lens::{Message, Model};

pub struct App {}

impl App {
    pub fn run(&self) -> std::io::Result<()> {
        let mut terminal = ratatui::init();
        let mut model = Model::initialize(projection());

        loop {
            terminal.draw(|f| view(&model, f))?;

            let message = handle_event(&model)?;

            if let Some(message) = message {
                model.update(message);
            }

            if model.is_exited() {
                break;
            }
        }
        ratatui::restore();

        Ok(())
    }
}

fn projection() -> Projection {
    let mut projection = Projection::default();
    let mut workspace = Workspace::new("Hermione".to_string());
    workspace.add_instruction(Instruction::new(InstructionAttributes {
        name: "Format project".to_string(),
        directive: "cargo fmt".to_string(),
    }));
    workspace.add_instruction(Instruction::new(InstructionAttributes {
        name: "Lint project".to_string(),
        directive: "cargo clippy".to_string(),
    }));
    projection.add_workspace(workspace);
    projection.add_workspace(Workspace::new("General".to_string()));

    let mut workspace = Workspace::new("Vulkan tutorial".to_string());
    workspace.add_instruction(Instruction::new(InstructionAttributes {
        name: "Compile shader fragment".to_string(),
        directive:
            r#"C:\VulkanSDK\1.3.290.0\Bin\glslc.exe .\shaders\shader.frag -o .\shaders\frag.spv"#
                .to_string(),
    }));
    projection.add_workspace(workspace);
    projection
}

fn view(model: &Model, frame: &mut Frame) {
    let layout = Layout::new(
        Direction::Horizontal,
        vec![Constraint::Percentage(25), Constraint::Percentage(75)],
    )
    .flex(Flex::Start);
    let [left, right] = layout.areas(frame.area());

    let list = List::new(model.workspace_names())
        .highlight_style(Style::new().reversed())
        .block(
            Block::new()
                .title("Workspaces")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
    let mut state = ListState::default();

    state.select(model.selected_workspace_index());

    frame.render_stateful_widget(list, left, &mut state);

    if let Some(index) = model.selected_workspace_index() {
        let list = List::new(model.workspace_commands(index)).block(
            Block::new()
                .title("Commands")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(list, right)
    } else {
        let block = Block::new()
            .title("Commands")
            .title_alignment(Alignment::Center)
            .borders(Borders::all());

        frame.render_widget(block, right)
    }
}

fn handle_event(model: &Model) -> std::io::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && matches!(key.code, KeyCode::Char('q')) {
                return Ok(Some(Message::Exit));
            }

            if key.kind == event::KeyEventKind::Press {
                let message = handle_key(key, model);

                return Ok(message);
            }
        }
    }

    Ok(None)
}

fn handle_key(key: event::KeyEvent, model: &Model) -> Option<Message> {
    match key.code {
        KeyCode::Down => {
            if model.workspace_names().is_empty() {
                return None;
            }

            if let Some(index) = model.selected_workspace_index() {
                if index < model.workspace_names().len() - 1 {
                    return Some(Message::SelectWorkspace(index + 1));
                }
            }

            Some(Message::SelectWorkspace(0))
        }

        KeyCode::Up => {
            if model.workspace_names().is_empty() {
                return None;
            }

            if let Some(index) = model.selected_workspace_index() {
                if index > 0 {
                    return Some(Message::SelectWorkspace(index - 1));
                }
            }

            Some(Message::SelectWorkspace(model.workspace_names().len() - 1))
        }
        _ => None,
    }
}
