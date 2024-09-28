use ratatui::style::{Style, Stylize};

#[derive(Default)]
pub struct Color {
    style: Style,
}

impl Color {
    pub fn highlight(self) -> Style {
        self.style.on_light_blue()
    }
}
