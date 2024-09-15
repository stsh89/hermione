pub mod command_center;
pub mod command_display;
pub mod lobby;
pub mod new_command;
pub mod new_workspace;

use crate::Result;
use ratatui::crossterm::event::{self, Event, KeyCode};

fn handle_event<T, M>(map_key_to_message: T) -> Result<Option<M>>
where
    T: FnOnce(KeyCode) -> Result<Option<M>>,
{
    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press {
            let message = map_key_to_message(key.code)?;

            return Ok(message);
        }
    }

    Ok(None)
}
