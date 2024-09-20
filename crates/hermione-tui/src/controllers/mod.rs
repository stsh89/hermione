pub mod change_location;
pub mod command_center;
pub mod command_display;
pub mod editor;
pub mod lobby;

use crate::{key_mappings::InputMode, Result};
use ratatui::crossterm::event::{self, Event, KeyEvent};

fn handle_event<T, M>(map_key_to_message: T, mode: InputMode) -> Result<Option<M>>
where
    T: FnOnce(KeyEvent, InputMode) -> Result<Option<M>>,
{
    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press {
            let message = map_key_to_message(key, mode)?;

            return Ok(message);
        }
    }

    Ok(None)
}

pub fn event_handler<F, M>(f: F) -> Result<Option<M>>
where
    F: Fn(KeyEvent) -> Option<M>,
{
    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press {
            let message = f(key);

            return Ok(message);
        }
    }

    Ok(None)
}
