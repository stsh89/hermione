use ratatui::style::Color;
use serde::Deserialize;

pub const HIGHLIGHT_SYMBOL: &str = "➤ ";

#[derive(Clone, Copy, Default, Deserialize)]
pub struct Theme {
    pub background_color: Color,
    pub danger_color: Color,
    pub foreground_color: Color,
    pub highlight_color: Color,
    pub input_color: Color,
    pub popup_background_color: Color,
    pub status_bar_background_color: Color,
    pub status_bar_foreground_color: Color,
}
