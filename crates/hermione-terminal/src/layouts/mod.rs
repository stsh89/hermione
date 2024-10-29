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

impl<'a> TryFrom<StatusBar<'a>> for String {
    type Error = serde_json::Error;

    fn try_from(value: StatusBar<'a>) -> Result<Self, Self::Error> {
        serde_json::to_string(&value)
    }
}
