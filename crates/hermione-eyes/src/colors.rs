use ratatui::style::Color as InnerColor;
use std::ops::Deref;

pub struct Color(InnerColor);

impl Color {
    pub fn highlight() -> Self {
        Self(InnerColor::LightYellow)
    }
}

impl Deref for Color {
    type Target = InnerColor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
