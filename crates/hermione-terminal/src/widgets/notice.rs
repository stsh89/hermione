use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

pub struct Notice<'a> {
    kind: NoticeKind,
    message: &'a str,
    border_style: Style,
    background_color: Color,
    foreground_color: Color,
}

pub enum NoticeKind {
    Error,
}

impl NoticeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            NoticeKind::Error => "Error",
        }
    }
}

impl<'a> Notice<'a> {
    pub fn error(message: &'a str) -> Self {
        Self {
            kind: NoticeKind::Error,
            message,
            border_style: Style::default(),
            background_color: Color::default(),
            foreground_color: Color::default(),
        }
    }

    pub fn set_border_style<S>(mut self, style: S) -> Self
    where
        S: Into<Style>,
    {
        self.border_style = style.into();
        self
    }

    pub fn set_background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn set_foreground_color(mut self, color: Color) -> Self {
        self.foreground_color = color;
        self
    }
}

impl<'a> Widget for Notice<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .borders(Borders::all())
            .title(self.kind.as_str())
            .border_style(self.border_style);

        let paragraph = Paragraph::new(self.message)
            .wrap(Wrap { trim: false })
            .bg(self.background_color)
            .block(block);

        paragraph.render(area, buf)
    }
}
