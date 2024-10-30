use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Styled},
    widgets::{Paragraph, Widget},
};
use serde::Serialize;

pub mod search_list;
pub mod wide;

#[derive(Default, Serialize)]
pub struct StatusBar<'a> {
    operation: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    workspace: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    selector: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pwsh: Option<&'a str>,
}

pub struct StatusBarWidget {
    text: String,
    style: Style,
}

impl<'a> StatusBar<'a> {
    pub fn operation(self, name: &'a str) -> Self {
        Self {
            operation: name,
            ..self
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

    pub fn page(self, page: u32) -> Self {
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

impl StatusBarWidget {
    pub fn new(text: String) -> Self {
        Self {
            text,
            style: Style::default(),
        }
    }

    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }
}

impl Styled for StatusBarWidget {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl Widget for StatusBarWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Paragraph::new(self.text)
            .style(self.style)
            .render(area, buf)
    }
}

impl<'a> TryFrom<StatusBar<'a>> for String {
    type Error = serde_json::Error;

    fn try_from(value: StatusBar<'a>) -> Result<Self, Self::Error> {
        serde_json::to_string(&value)
    }
}
