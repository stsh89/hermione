use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph, Widget},
};

#[derive(Default)]
pub struct FormField<'a> {
    background_color: Color,
    foreground_color: Color,
    name: &'a str,
    value: &'a str,
}

impl<'a> FormField<'a> {
    pub fn set_background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn set_foreground_color(mut self, color: Color) -> Self {
        self.foreground_color = color;
        self
    }

    pub fn name(self, title: &'a str) -> Self {
        Self {
            name: title,
            ..self
        }
    }

    pub fn value(self, value: &'a str) -> Self {
        Self { value, ..self }
    }
}

impl<'a> Widget for FormField<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.name)
            .bg(self.background_color)
            .fg(self.foreground_color);

        let paragraph = Paragraph::new(self.value).block(block);

        paragraph.render(area, buf);
    }
}
