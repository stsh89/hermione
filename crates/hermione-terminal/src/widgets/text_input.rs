use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Styled},
    widgets::{Block, Paragraph, Widget},
};

pub struct TextInputWidget<'a> {
    style: Style,
    text: &'a str,
    block: Option<Block<'a>>,
}

impl<'a> TextInputWidget<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            style: Style::default(),
            text,
            block: None,
        }
    }

    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> Styled for TextInputWidget<'a> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl<'a> Widget for TextInputWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut paragraph = Paragraph::new(self.text).style(self.style);

        if let Some(block) = self.block {
            paragraph = paragraph.block(block);
        }

        paragraph.render(area, buf)
    }
}
