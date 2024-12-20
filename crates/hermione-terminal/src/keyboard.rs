#[derive(Clone, Copy)]
pub enum Event {
    Backspace,
    Char(char),
    Down,
    Enter,
    Esc,
    Space,
    Up,
}

pub fn read_event() -> anyhow::Result<Event> {
    use ratatui::crossterm::event;

    loop {
        let event = event::read()?;

        if let event::Event::Key(key) = event {
            if key.kind == event::KeyEventKind::Press {
                let event = match key.code {
                    event::KeyCode::Backspace => Event::Backspace,
                    event::KeyCode::Char(' ') => Event::Space,
                    event::KeyCode::Char(c) => Event::Char(c),
                    event::KeyCode::Down => Event::Down,
                    event::KeyCode::Enter => Event::Enter,
                    event::KeyCode::Esc => Event::Esc,
                    event::KeyCode::Up => Event::Up,
                    _ => continue,
                };

                return Ok(event);
            }
        }
    }
}
