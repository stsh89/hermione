mod github_dark;

use crate::{widgets::StatusBarWidget, widgets::TextInputWidget};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, List},
};

pub trait Themed {
    fn themed(self, theme: Theme) -> Self;
}

#[derive(Clone, Copy)]
pub struct Theme {
    background_color: Color,
    foreground_color: Color,
    highlight_color: Color,
    highlight_symbol: &'static str,
    input_color: Color,
    status_bar_background_color: Color,
    status_bar_foreground_color: Color,
}

impl Theme {
    pub fn github_dark() -> Self {
        Theme {
            background_color: github_dark::BACKGROUND_COLOR.into(),
            foreground_color: github_dark::FOREGROUND_COLOR.into(),
            highlight_color: github_dark::HIGHLIGHT_COLOR.into(),
            highlight_symbol: github_dark::HIGHLIGHT_SYMBOL,
            input_color: github_dark::INPUT_COLOR.into(),
            status_bar_background_color: github_dark::STATUS_BAR_BACKGROUND_COLOR.into(),
            status_bar_foreground_color: github_dark::STATUS_BAR_FOREGROUND_COLOR.into(),
        }
    }
}

impl Themed for Block<'_> {
    fn themed(self, theme: Theme) -> Self {
        self.style(
            Style::default()
                .bg(theme.background_color)
                .fg(theme.foreground_color),
        )
    }
}

impl Themed for List<'_> {
    fn themed(self, theme: Theme) -> Self {
        self.highlight_style(theme.highlight_color)
            .highlight_symbol(theme.highlight_symbol)
    }
}

impl Themed for TextInputWidget<'_> {
    fn themed(self, theme: Theme) -> Self {
        self.style(Style::default().fg(theme.input_color))
    }
}

impl Themed for StatusBarWidget<'_> {
    fn themed(self, theme: Theme) -> Self {
        self.style(
            Style::default()
                .bg(theme.status_bar_background_color)
                .fg(theme.status_bar_foreground_color),
        )
    }
}
