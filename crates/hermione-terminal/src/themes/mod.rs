use crate::Result;
use ratatui::style::Color;
use serde::Deserialize;

pub const HIGHLIGHT_SYMBOL: &str = "➤ ";

#[derive(Clone, Copy, Default)]
pub struct Theme {
    pub background_color: Color,
    pub foreground_color: Color,
    pub highlight_color: Color,
    pub input_color: Color,
    pub popup_background_color: Color,
    pub danger_color: Color,
    pub status_bar_background_color: Color,
    pub status_bar_foreground_color: Color,
}

#[derive(Deserialize)]
struct JsonTheme {
    background_color: String,
    danger_color: String,
    foreground_color: String,
    highlight_color: String,
    input_color: String,
    popup_background_color: String,
    status_bar_background_color: String,
    status_bar_foreground_color: String,
}

impl Theme {
    pub fn parse(json: &str) -> Result<Self> {
        let theme = serde_json::from_str::<JsonTheme>(json)?;

        Ok(Theme {
            background_color: theme.background_color.try_from_hex()?,
            danger_color: theme.danger_color.try_from_hex()?,
            foreground_color: theme.foreground_color.try_from_hex()?,
            highlight_color: theme.highlight_color.try_from_hex()?,
            input_color: theme.input_color.try_from_hex()?,
            popup_background_color: theme.popup_background_color.try_from_hex()?,
            status_bar_background_color: theme.status_bar_background_color.try_from_hex()?,
            status_bar_foreground_color: theme.status_bar_foreground_color.try_from_hex()?,
        })
    }
}

trait TryFromHex {
    fn try_from_hex(self) -> Result<Color>;
}

impl TryFromHex for String {
    fn try_from_hex(self) -> Result<Color> {
        Ok(Color::Rgb(
            u8::from_str_radix(&self[1..3], 16)?,
            u8::from_str_radix(&self[3..5], 16)?,
            u8::from_str_radix(&self[5..7], 16)?,
        ))
    }
}
