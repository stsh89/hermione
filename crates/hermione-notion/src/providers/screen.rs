use std::io;

use hermione_ops::{
    notion::{Credentials, CredentialsParameters, GetCredentials},
    Result,
};
use serde::Serialize;

pub struct ScreenProvider;

#[derive(Serialize)]
pub struct CredentialsStream {
    pub api_key: String,
    pub commands_page_id: String,
    pub workspaces_page_id: String,
}

impl ScreenProvider {
    pub fn ask(&self, prompt: &str) -> io::Result<String> {
        clear_screen_and_reset_cursor();

        let mut buf = String::new();

        use std::io::Write;
        print!("{prompt}");
        std::io::stdout().flush()?;

        std::io::stdin().read_line(&mut buf)?;

        Ok(buf.trim().to_string())
    }

    fn enter_credentials(&self) -> io::Result<CredentialsStream> {
        Ok(CredentialsStream {
            api_key: self.ask("Enter your Notion API key: ")?,
            commands_page_id: self.ask("Enter your Notion commands page ID: ")?,
            workspaces_page_id: self.ask("Enter your Notion workspaces page ID: ")?,
        })
    }

    pub fn new() -> Self {
        Self
    }

    pub fn show_credentials(&self, credentials: CredentialsStream) -> serde_json::Result<()> {
        let json_string = serde_json::to_string_pretty(&credentials)?;

        println!("{json_string}");

        Ok(())
    }
}

fn clear_screen_and_reset_cursor() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char)
}

impl From<Credentials> for CredentialsStream {
    fn from(value: Credentials) -> Self {
        Self {
            api_key: value.api_key().into(),
            commands_page_id: value.commands_page_id().into(),
            workspaces_page_id: value.workspaces_page_id().into(),
        }
    }
}

impl From<CredentialsStream> for Credentials {
    fn from(value: CredentialsStream) -> Self {
        let CredentialsStream {
            api_key,
            commands_page_id,
            workspaces_page_id,
        } = value;

        Self::new(CredentialsParameters {
            api_key,
            commands_page_id,
            workspaces_page_id,
        })
    }
}

impl GetCredentials for ScreenProvider {
    fn get_credentials(&self) -> Result<Credentials> {
        let credentials = self.enter_credentials()?;

        Ok(credentials.into())
    }
}
