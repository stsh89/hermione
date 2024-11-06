use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Styled},
    widgets::{Paragraph, Widget},
};
use serde::Serialize;
use std::{marker::PhantomData, num::NonZeroU32};

pub struct StatusBar {
    status_line_text: String,
}

pub struct StatusBarBuilder<'a, O> {
    command: Option<&'a str>,
    operation: Option<&'a str>,
    page: Option<NonZeroU32>,
    phantom_data: PhantomData<O>,
    pwsh: Option<&'a str>,
    search: Option<&'a str>,
    selector: Option<&'a str>,
    workspace: Option<&'a str>,
}

pub struct StatusBarWidget<'a> {
    state: &'a StatusBar,
    style: Style,
}

#[derive(Serialize)]
pub struct StatusLine<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<&'a str>,

    operation: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<NonZeroU32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pwsh: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    selector: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    workspace: Option<&'a str>,
}

pub struct Set;
pub struct Unset;

impl StatusBar {
    pub fn builder<'a>() -> StatusBarBuilder<'a, Unset> {
        StatusBarBuilder::default()
    }
}

impl<'a> StatusBarBuilder<'a, Unset> {
    pub fn operation(self, operation: &'a str) -> StatusBarBuilder<'a, Set> {
        let StatusBarBuilder {
            operation: _,
            workspace,
            command,
            selector,
            search,
            page,
            pwsh,
            phantom_data: _,
        } = self;

        StatusBarBuilder {
            operation: Some(operation),
            workspace,
            command,
            selector,
            search,
            page,
            pwsh,
            phantom_data: PhantomData,
        }
    }
}

impl<'a> StatusBarBuilder<'a, Set> {
    pub fn build(self) -> StatusBar {
        let Self {
            command,
            operation,
            page,
            phantom_data: _,
            pwsh,
            search,
            selector,
            workspace,
        } = self;

        let status_line = StatusLine {
            command,
            operation: operation.unwrap(),
            page,
            pwsh,
            search,
            selector,
            workspace,
        };

        StatusBar {
            status_line_text: serde_json::to_string(&status_line).unwrap_or_default(),
        }
    }

    pub fn workspace(self, workspace: &'a str) -> Self {
        Self {
            workspace: Some(workspace),
            ..self
        }
    }

    pub fn command(self, command: &'a str) -> Self {
        Self {
            command: Some(command),
            ..self
        }
    }

    pub fn page(self, page: NonZeroU32) -> Self {
        Self {
            page: Some(page),
            ..self
        }
    }

    pub fn selector(self, selector: &'a str) -> Self {
        Self {
            selector: Some(selector),
            ..self
        }
    }

    pub fn pwsh(self, pwsh: &'a str) -> Self {
        Self {
            pwsh: Some(pwsh),
            ..self
        }
    }

    pub fn search(self, search: &'a str) -> Self {
        Self {
            search: Some(search),
            ..self
        }
    }
}

impl<'a> Default for StatusBarBuilder<'a, Unset> {
    fn default() -> Self {
        Self {
            operation: None,
            workspace: None,
            command: None,
            selector: None,
            search: None,
            page: None,
            pwsh: None,
            phantom_data: PhantomData,
        }
    }
}

impl<'a> StatusBarWidget<'a> {
    pub fn new(state: &'a StatusBar) -> Self {
        Self {
            state,
            style: Style::default(),
        }
    }

    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }
}

impl Styled for StatusBarWidget<'_> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl Widget for StatusBarWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Paragraph::new(self.state.status_line_text.as_str())
            .style(self.style)
            .render(area, buf)
    }
}
