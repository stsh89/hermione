use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyCode}, layout::{Constraint, Direction, Layout, Rect}, widgets::Paragraph, Frame
};

use lense::{Message, Model};

pub struct App {}

impl App {
    pub fn run(&self) -> std::io::Result<()> {
        let mut terminal = ratatui::init();
        let mut model = Model::default();

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

fn view(_model: &mut Model, frame: &mut Frame) {
    let layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Length(5), Constraint::Min(0)],
    )
    .split(Rect::new(0, 0, 10, 10));
    frame.render_widget(Paragraph::new("foo"), layout[0]);
    frame.render_widget(Paragraph::new("bar"), layout[1]);
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
        _ => None,
    }
}
