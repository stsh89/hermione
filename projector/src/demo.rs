use std::time::Duration;

use projection::{Instruction, InstructionAttributes, Projection, Workspace};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Alignment, Constraint, Direction, Flex, Layout, Position},
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
    if model.new_workspace_is_prepared() {
        new_workspace_view(model, frame);
        return;
    }

    if model.workspace_is_entered() {
        workspace_view(model, frame);
        return;
    }

    workspaces_view(model, frame);
}

fn new_workspace_view(model: &Model, frame: &mut Frame) {
    let layout = Layout::new(
        Direction::Vertical,
        vec![Constraint::Percentage(100)],
    ).flex(Flex::Start);

    let [top] = layout.areas(frame.area());

    let input = model.new_workspace_input().unwrap();
    let paragraph = Paragraph::new(input.value()).block(
        Block::new()
            .title("Enter workspace name")
            .title_alignment(Alignment::Center)
            .borders(Borders::all()),
    );

    frame.render_widget(paragraph, top);

    frame.set_cursor_position(Position::new(
        // Draw the cursor at the current position in the input field.
        // This position is can be controlled via the left and right arrow key
        top.x + input.character_index() as u16 + 1,
        // Move one line down, from the border to the input line
        top.y + 1,
    ));
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
    // if let Some(mut input) = model.new_workspace_input() {
    //     let message = match key.code {
    //         KeyCode::Enter => input.submit_message(),
    //         KeyCode::Char(to_insert) => input.enter_char(to_insert),
    //         KeyCode::Backspace => input.delete_char(),
    //         KeyCode::Left => input.move_cursor_left(),
    //         KeyCode::Right => input.move_cursor_right(),
    //         KeyCode::Esc => Some(Message::CancelNewWorkspace),
    //         _ => None
    //     };

    //     return message;
    // }


    match key.code {
        KeyCode::Char('n') => {
            if model.workspace_is_entered() {
                return None;
            }

            Some(Message::NewWorkspace)
        }

        KeyCode::Char('d') => {
            if model.workspace_is_entered() {
                return None;
            }

            if model.workspace_is_selected() {
                return Some(Message::DeleteWorkspace);
            }

            None
        }

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
            } else if model.new_workspace_is_prepared() {
                Some(Message::CancelNewWorkspace)
            } else if model.workspace_is_selected() {
                Some(Message::UnselectWorkspace)
            } else {
                Some(Message::Exit)
            }
        }
        _ => None,
    }
}
