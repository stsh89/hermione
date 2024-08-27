use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState},
    Frame,
};

use lense::{Message, Model};

pub struct App {}

impl App {
    pub fn run(&self) -> std::io::Result<()> {
        let mut terminal = ratatui::init();
        let mut model = Model::initialize();

        loop {
            terminal.draw(|f| view(&mut model, f))?;

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

fn view(model: &mut Model, frame: &mut Frame) {
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

    state.select(model.current_workspace_preview_index());

    frame.render_stateful_widget(list, left, &mut state);
    frame.render_widget(
        Block::new()
            .title("Instructions")
            .title_alignment(Alignment::Center)
            .borders(Borders::all()),
        right,
    )
}

fn handle_event(_: &Model) -> std::io::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }

    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('n') => Some(Message::NewWorkspace),
        KeyCode::Char('q') => Some(Message::Exit),
        KeyCode::Down => Some(Message::PreviewNextWorkspace),
        KeyCode::Up => Some(Message::PreviewPreviousWorkspace),
        _ => None,
    }
}
