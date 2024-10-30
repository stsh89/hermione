mod github_dark;

use ratatui::{
    style::Color,
    style::Style,
    widgets::{Block, Paragraph},
};

use crate::layouts::StatusBarWidget;

pub trait Themed {
    fn themed(self, theme: &Theme) -> Self;
}

pub struct Theme {
    background: Color,
    status_bar_background: Color,
}

impl Theme {
    pub fn github_dark() -> Self {
        Theme {
            background: github_dark::BACKGROUND_COLOR.into(),
            status_bar_background: github_dark::STATUS_BAR_BACKGROUND.into(),
        }
    }
}

impl Themed for Block<'_> {
    fn themed(self, theme: &Theme) -> Self {
        self.style(Style::default().bg(theme.background))
    }
}

impl Themed for Paragraph<'_> {
    fn themed(self, theme: &Theme) -> Self {
        self.style(Style::default().bg(theme.background))
    }
}

impl Themed for StatusBarWidget {
    fn themed(self, theme: &Theme) -> Self {
        self.style(Style::default().bg(theme.status_bar_background))
    }
}
