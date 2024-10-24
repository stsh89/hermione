use hermione_ops::{
    notion::{Credentials, CredentialsParameters, GetCredentials},
    Result,
};

pub struct ScreenProvider {
    stdin: std::io::Stdin,
}

impl ScreenProvider {
    pub fn new() -> Self {
        Self {
            stdin: std::io::stdin(),
        }
    }

    pub fn ask(&self, prompt: &str) -> Result<String> {
        // Clear screen and reset cursor
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        let mut buf = String::new();

        use std::io::Write;
        print!("{prompt}");
        std::io::stdout().flush()?;

        self.stdin.read_line(&mut buf)?;

        Ok(buf.trim().to_string())
    }

    pub fn print(&self, prompt: &str, text: &str) {
        println!("{prompt}{text}");
    }
}

impl GetCredentials for ScreenProvider {
    fn get(&self) -> Result<Credentials> {
        let api_key = self.ask("Enter your Notion API key: ")?;
        let commands_page_id = self.ask("Enter your Notion commands page ID: ")?;
        let workspaces_page_id = self.ask("Enter your Notion workspaces page ID: ")?;

        Ok(Credentials::new(CredentialsParameters {
            api_key,
            commands_page_id,
            workspaces_page_id,
        }))
    }
}
