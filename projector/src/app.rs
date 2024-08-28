use std::time::Duration;

use projection::{Instruction, InstructionAttributes, Projection, Workspace};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState, Paragraph},
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
    if model.workspace_is_entered() {
        workspace_view(model, frame);
    } else {
        workspaces_view(model, frame);
    }
}

fn workspaces_view(model: &Model, frame: &mut Frame) {
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

    let list = List::new(model.selected_workspace_commands()).block(
        Block::new()
            .title("Commands")
            .title_alignment(Alignment::Center)
            .borders(Borders::all()),
    );
    frame.render_widget(list, right)
}

fn workspace_view(model: &Model, frame: &mut Frame) {
    let layout = Layout::new(
        Direction::Vertical,
        vec![Constraint::Percentage(100), Constraint::Min(3)],
    )
    .flex(Flex::Start);
    let [top, bottom] = layout.areas(frame.area());

    let list = List::new(model.selected_workspace_commands())
        .highlight_style(Style::new().reversed())
        .block(
            Block::new()
                .title(format!(
                    "{} commands",
                    model.selected_workspace().unwrap().name()
                ))
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
    let mut state = ListState::default();

    state.select(model.selected_command_index());

    frame.render_stateful_widget(list, top, &mut state);

    let paragraph_text = if let Some(command) = model.selected_command() {
        command.name()
    } else {
        ""
    };

    let paragraph = Paragraph::new(paragraph_text).block(
        Block::new()
            .title("Command name")
            .title_alignment(Alignment::Center)
            .borders(Borders::all()),
    );
    frame.render_widget(paragraph, bottom)
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
            if model.workspace_is_entered() {
                if model.selected_workspace_commands().is_empty() {
                    return None;
                }

                if let Some(index) = model.selected_command_index() {
                    if index < model.selected_workspace_commands().len() - 1 {
                        return Some(Message::SelectCommand(index + 1));
                    }
                }

                return Some(Message::SelectCommand(0));
            }

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
            if model.workspace_is_entered() {
                if model.selected_workspace_commands().is_empty() {
                    return None;
                }

                if let Some(index) = model.selected_command_index() {
                    if index > 0 {
                        return Some(Message::SelectCommand(index - 1));
                    }
                }

                return Some(Message::SelectCommand(
                    model.selected_workspace_commands().len() - 1,
                ));
            }

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

        KeyCode::Enter => {
            if model.workspace_is_selected() {
                Some(Message::EnterWorkspace)
            } else {
                None
            }
        }

        KeyCode::Esc => {
            if model.workspace_is_entered() {
                Some(Message::ExitWorkspace)
            } else if model.workspace_is_selected() {
                Some(Message::UnselectWorkspace)
            } else {
                Some(Message::Exit)
            }
        }
        _ => None,
    }
}
