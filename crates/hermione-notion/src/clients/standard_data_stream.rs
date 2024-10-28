use serde::Serialize;
use std::io::{self, Result, Stdin};

pub struct StandardDataStreamClient {
    stdin: Stdin,
}

#[derive(Serialize)]
pub struct CredentialsStream {
    pub api_key: String,
    pub commands_page_id: String,
    pub workspaces_page_id: String,
}

impl StandardDataStreamClient {
    pub fn new() -> Self {
        Self { stdin: io::stdin() }
    }

    pub fn ask(&self, prompt: &str) -> Result<String> {
        clear_screen_and_reset_cursor();

        let mut buf = String::new();

        use std::io::Write;
        print!("{prompt}");
        std::io::stdout().flush()?;

        self.stdin.read_line(&mut buf)?;

        Ok(buf.trim().to_string())
    }

    pub fn read_credentials(&self) -> Result<CredentialsStream> {
        Ok({
            CredentialsStream {
                api_key: self.ask("Enter your Notion API key: ")?,
                commands_page_id: self.ask("Enter your Notion commands page ID: ")?,
                workspaces_page_id: self.ask("Enter your Notion workspaces page ID: ")?,
            }
        })
    }

    pub fn write_credentials(&self, credentials: CredentialsStream) -> Result<()> {
        let json_string = serde_json::to_string_pretty(&credentials)?;

        println!("{json_string}");

        Ok(())
    }
}

fn clear_screen_and_reset_cursor() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char)
}
