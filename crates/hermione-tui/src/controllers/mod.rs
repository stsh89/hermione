pub mod command_center;
pub mod command_display;
pub mod lobby;
pub mod new_command;
pub mod new_workspace;

use crate::{key_mappings::InputMode, Result};
use ratatui::crossterm::event::{self, Event, KeyCode};

fn handle_event<T, M>(map_key_to_message: T, mode: InputMode) -> Result<Option<M>>
where
    T: FnOnce(KeyCode, InputMode) -> Result<Option<M>>,
{
    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press {
            let message = map_key_to_message(key.code, mode)?;

            return Ok(message);
        }
    }

    Ok(None)
}
